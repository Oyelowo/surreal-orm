import * as k8s from '@pulumi/kubernetes';
import * as kx from '@pulumi/kubernetesx';
import * as pulumi from '@pulumi/pulumi';
import crds from '../../../generatedCrdsTs/index.js';
import { DOCKER_REGISTRY_KEY } from '../infrastructure/argocd/docker.js';
import { createArgocdApplication } from './createArgoApplication.js';
import { getResourceAbsolutePath } from './directoriesManager.js';
import { AppConfigs, DBType, NamespaceOfApps, NoUnion, ServiceName } from '../types/ownTypes.js';
import { getEnvironmentVariables } from './validations.js';
import { generateService } from './helpers.js';
import { toBase64 } from './converters.js';
import _ from 'lodash';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

// eslint-disable-next-line no-restricted-syntax
export class ServiceDeployment<
    AN extends ServiceName,
    DBT extends DBType,
    NS extends NamespaceOfApps
> extends pulumi.ComponentResource {
    public readonly deployment: kx.Deployment;
    public readonly configMaps: kx.ConfigMap;
    public readonly secret: kx.Secret;
    public readonly service: kx.Service;
    public readonly argocdApplication: crds.argoproj.v1alpha1.Application;
    public readonly ipAddress?: pulumi.Output<string>;
    public readonly provider?: pulumi.ProviderResource;
    public readonly secretProvider?: pulumi.ProviderResource;
    public readonly appName: AN;

    constructor(
        private name: NoUnion<AN>,
        private args: AppConfigs<AN, DBT, NS> // opts: pulumi.ComponentResourceOptions
    ) {
        super('k8sjs:service:ServiceDeployment', name, {} /* opts */);
        this.appName = name;
        const { envVars, kubeConfig } = args;
        const metadata = {
            ...args.metadata,
        };
        const resourceName = metadata.name;

        this.provider = new k8s.Provider(
            this.appName,
            {
                renderYamlToDirectory: this.getServiceDir(),
            },
            { parent: this }
        );

        this.configMaps = new kx.ConfigMap(
            `${resourceName}-configmap`,
            {
                data: { config: 'very important data' },
                metadata,
            },
            { provider: this.getProvider(), parent: this }
        );

        // const plainSecrets = new PlainSecretJsonConfig(this.appName, ENVIRONMENT).getSecrets();
        const encodedSecrets = _.mapValues(envVars, toBase64) as typeof envVars;

        // Create a Kubernetes Secret.
        this.secret = new kx.Secret(
            `${resourceName}-secret`,
            {
                data: encodedSecrets,
                metadata: {
                    ...metadata,
                    annotations: {
                        'sealedsecrets.bitnami.com/managed': 'true',
                    },
                },
            },
            { provider: this.getProvider(), parent: this }
        );

        // Define a Pod.
        const podBuilder = new kx.PodBuilder({
            initContainers: [],
            containers: [
                {
                    env: {
                        ...this.secretsObjectToEnv({ secretInstance: this.secret, secretObject: envVars }),
                    },
                    image: kubeConfig.image,
                    ports: { http: Number(envVars.APP_PORT) },
                    volumeMounts: [],
                    resources: {
                        limits: {
                            memory: kubeConfig.limitMemory,
                            cpu: kubeConfig.limitCpu,
                        },
                        requests: {
                            memory: kubeConfig.requestMemory,
                            cpu: kubeConfig.requestCpu,
                        },
                    },
                    ...(kubeConfig.readinessProbePort && {
                        readinessProbe: {
                            httpGet: {
                                // scheme: "http",
                                path: '/api/healthz',
                                port: kubeConfig.readinessProbePort,
                            },
                            initialDelaySeconds: 60,
                            periodSeconds: 10,
                            failureThreshold: 7,
                        },
                        // Something to consider: Use a different strategy for this. This endpoint checks our db or redis
                        // and wont be nice if either is down. We still want to be able to show users that
                        // the application is unavailable
                        livenessProbe: {
                            httpGet: {
                                path: '/api/liveness',
                                port: kubeConfig.readinessProbePort,
                            },
                            initialDelaySeconds: 300,
                            periodSeconds: 10,
                            failureThreshold: 7,
                        },
                        // startupProbe
                    }),
                },
            ],
            ...(ENVIRONMENT !== 'local' && {
                securityContext: {
                    runAsNonRoot: true,
                    runAsUser: 10_000,
                    runAsGroup: 10_000,
                    // fsGroup:
                },
            }),
            imagePullSecrets: [
                {
                    name: DOCKER_REGISTRY_KEY,
                },
            ],
        });

        // Create a Kubernetes Deployment.
        this.deployment = new kx.Deployment(
            `${resourceName}-deployment`,
            {
                spec: podBuilder.asDeploymentSpec({
                    replicas: kubeConfig.replicaCount,
                }),
                metadata: {
                    ...metadata,
                    annotations: {
                        'linkerd.io/inject': 'enabled',
                    },
                },
            },
            { provider: this.getProvider(), parent: this }
        );

        this.service = generateService({
            deployment: this.deployment,
            serviceFileName: `${resourceName}-service`,
            args: {
                type: kx.types.ServiceType.ClusterIP,
                ports: [
                    {
                        port: Number(envVars.APP_PORT),
                        protocol: 'TCP',
                        name: `${resourceName}-http`,
                        targetPort: Number(envVars.APP_PORT),
                    },
                ],
            },
        });
        // this.service = this.deployment.createService({
        //     type: kx.types.ServiceType.ClusterIP,
        //     ports: [
        //         {
        //             port: Number(envVars.APP_PORT),
        //             protocol: 'TCP',
        //             name: `${resourceName}-http`,
        //             targetPort: Number(envVars.APP_PORT),
        //         },
        //     ],
        // });

        this.argocdApplication = createArgocdApplication({
            sourceAppDirectory: `services/${this.appName}`,
            outputDirectory: `infrastructure/argocd-applications-children-services`,
            environment: ENVIRONMENT,
            namespace: metadata.namespace,
            parent: this,
        });

        const useLoadBalancer = new pulumi.Config('useLoadBalancer') ?? false;
        this.ipAddress = useLoadBalancer ? this.service.status.loadBalancer.ingress[0].ip : this.service.spec.clusterIP;
    }

    /**
     Maps custom secret object to what kx can understand to produce secretRef automagically
     * @example
     {
        "graphql-mongo": {
            MONGODB_USERNAME: "xxxx",
            MONGODB_PASSWORD: "xxxx",
            REDIS_USERNAME: "xxxx",
            REDIS_PASSWORD: "xxxx",
        };
        "graphql-postgres": {
            POSTGRES_USERNAME: "xxxx",
            POSTGRES_PASSWORD: "xxxx",
        };
     }
     
     to
     {
        MONGODB_USERNAME:
            secretRef:
              ...
      ...
     }
    */
    private secretsObjectToEnv = ({
        secretInstance,
        secretObject,
    }: {
        secretInstance: kx.Secret;
        secretObject: AppConfigs<AN, DBT, NS>['envVars'];
    }) => {
        const keyValueEntries = Object.keys(secretObject).map((key) => [key, secretInstance.asEnvValue(key)]);
        return Object.fromEntries(keyValueEntries);
    };

    getProvider = () => {
        return this.provider;
    };

    getServiceDir = (): string => {
        return getResourceAbsolutePath({
            outputDirectory: `services/${this.appName}`,
            environment: ENVIRONMENT,
        });
    };
}

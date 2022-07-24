import * as k8s from '@pulumi/kubernetes';
import * as kx from '@pulumi/kubernetesx';
import * as pulumi from '@pulumi/pulumi';
import * as argocd from '../../crds-generated/argoproj';
import { DOCKER_REGISTRY_KEY } from './../infrastructure/argocd/docker';
import { createArgocdApplication } from './createArgoApplication';
import { getPathToResource } from './manifestsDirectory';
import { AppConfigs, DBType, NamespaceOfApps, NoUnion, ServiceName } from '../types/own-types';
import { getEnvironmentVariables } from './validations';
import { generateService } from './helpers';
import { PlainSecretJsonConfig } from '../../scripts/utils/plainSecretJsonConfig';

const { ENVIRONMENT } = getEnvironmentVariables();

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
    public readonly argocdApplication: argocd.v1alpha1.Application;
    public readonly ipAddress?: pulumi.Output<string>;
    public readonly provider?: pulumi.ProviderResource;
    public readonly secretProvider?: pulumi.ProviderResource;
    public readonly appName: AN;

    constructor(
        name: NoUnion<AN>,
        args: AppConfigs<AN, DBT, NS>
        // opts: pulumi.ComponentResourceOptions
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

        const secrets = new PlainSecretJsonConfig(this.appName, ENVIRONMENT);
        // Create a Kubernetes Secret.
        this.secret = new kx.Secret(
            `${resourceName}-secret`,
            {
                stringData: {
                    ...secrets,
                },
                metadata: {
                    ...metadata,
                    annotations: {
                        // 'sealedsecrets.bitnami.com/managed': 'true',
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
                        ...envVars,
                        ...this.#secretsObjectToEnv(this.secret),
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
            securityContext: {
                runAsNonRoot: true,
                runAsUser: 10000,
                runAsGroup: 10000,
                // fsGroup:
            },
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
            sourceApplication: this.appName,
            outputSubDirName: 'argocd-applications-children-services',
            namespace: metadata.namespace,
            parent: this,
        });

        const useLoadBalancer = new pulumi.Config('useLoadBalancer') ?? false;
        if (useLoadBalancer) {
            this.ipAddress = this.service.status.loadBalancer.ingress[0].ip;
        } else {
            this.ipAddress = this.service.spec.clusterIP;
        }
    }

    /**
     Maps custom secret object to what kx can understand to produce secretRef automagically
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
    #secretsObjectToEnv = (secretInstance: kx.Secret) => {
        const secretObject = new PlainSecretJsonConfig(this.appName, ENVIRONMENT).getSecrets();
        const keyValueEntries = Object.keys(secretObject).map((key) => [key, secretInstance.asEnvValue(key)]);
        return Object.fromEntries(keyValueEntries);
    };

    getProvider = () => {
        return this.provider;
    };

    getServiceDir = (): string => {
        return getPathToResource({
            resourceType: 'services',
            environment: ENVIRONMENT,
            resourceName: this.appName,
        });
    };
}

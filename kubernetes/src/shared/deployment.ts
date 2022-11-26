import * as k8s from '@pulumi/kubernetes';
import * as kx from '@pulumi/kubernetesx';
import * as pulumi from '@pulumi/pulumi';
import crds from '../../generatedCrdsTs/index.js';
import { DOCKER_REGISTRY_KEY } from '../infrastructure/argocd/docker.js';
import { createArgocdApplication } from '../infrastructure/argocd/createArgoApplication.js';
import { getResourceAbsolutePath } from './directoriesManager.js';
import { AppConfigs, NamespaceOfApps, NoUnion, ServiceName } from '../types/ownTypes.js';
import { generateService } from './helpers.js';
import { toBase64 } from './helpers.js';
import _ from 'lodash';
import z from 'zod';
import { getEnvVarsForKubeManifests } from './environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();


export class ServiceDeployment<N extends ServiceName, NS extends NamespaceOfApps> extends pulumi.ComponentResource {
    public readonly deployment: kx.Deployment;
    public readonly configMaps: kx.ConfigMap;
    public readonly secret: kx.Secret;
    public readonly service: kx.Service;
    public readonly argocdApplication: crds.argoproj.v1alpha1.Application;
    public readonly ipAddress?: pulumi.Output<string>;
    public provider: pulumi.ProviderResource;
    public readonly secretProvider?: pulumi.ProviderResource;
    public readonly appName: N;

    constructor(
        private name: NoUnion<N>,
        private args: AppConfigs<N, NS, Record<string, string>> // opts: pulumi.ComponentResourceOptions
    ) {
        super('k8sjs:service:ServiceDeployment', name, {} /* opts */);
        this.appName = name;
        const { envVars, kubeConfig } = args;
        const APP_PORT = z.string().parse(envVars.APP_PORT);
        const encodedSecrets = z.record(z.string()).parse(_.mapValues(envVars, toBase64));
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
                    /**
 Maps custom secret object to what kx can understand to produce secretRef automagically
 * @example
 {
    SURREALDB_USERNAME: "xxxx",
    SURREALDB_PASSWORD: "xxxx",
 }
 
 to
 {
    SURREALDB_USERNAME:
        secretRef:
          ...
  ...
 }
*/
                    env: _.mapKeys(encodedSecrets, (key) => [key, this.secret.asEnvValue(key)]),
                    image: kubeConfig.image,
                    ports: { http: Number(APP_PORT) },
                    volumeMounts: [],
                    command: kubeConfig.command,
                    args: kubeConfig.commandArgs,
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
                        port: Number(APP_PORT),
                        protocol: 'TCP',
                        name: `${resourceName}-http`,
                        targetPort: Number(APP_PORT),
                    },
                ],
            },
        });
        // this.service = this.deployment.createService({
        //     type: kx.types.ServiceType.ClusterIP,
        //     ports: [
        //         {
        //             port: Number(APP_PORT),
        //             protocol: 'TCP',
        //             name: `${resourceName}-http`,
        //             targetPort: Number(APP_PORT),
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

    getProvider = () => this.provider;
    setProvider = (provider: NonNullable<typeof this.provider>): this => {
        this.provider = provider;
        return this;
    };
    getServiceDir = (): string => {
        return getResourceAbsolutePath({
            outputDirectory: `services/${this.appName}`,
            environment: ENVIRONMENT,
        });
    };
}

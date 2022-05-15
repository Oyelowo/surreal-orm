import { DOCKER_REGISTRY_KEY } from './../infrastructure/argocd/docker';
import { getEnvironmentVariables } from "./validations";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";
import { NoUnion } from "./types/own-types";
import {
  AppConfigs,
  AppName,
  DBType,
  NamespaceOfApps,
} from "./types/own-types";
import * as argocd from "../../crd2pulumi/argocd";
import { createArgocdChildrenApplication } from "./createArgoApplication";
import {
  getPathToResource,
} from "./manifestsDirectory";
import { getSecretsForApp } from "../../scripts/secretsManagement/getSecretsForApp";
import { APPLICATION_AUTOMERGE_ANNOTATION } from './constants';




const { ENVIRONMENT } = getEnvironmentVariables()
export class ServiceDeployment<
  AN extends AppName,
  DBT extends DBType,
  NS extends NamespaceOfApps
  > extends pulumi.ComponentResource {
  public readonly deployment: kx.Deployment;
  public readonly configMaps: kx.ConfigMap;
  public readonly secret: kx.Secret;
  public readonly service: kx.Service;
  public readonly argocdApplication: argocd.argoproj.v1alpha1.Application;
  public readonly ipAddress?: pulumi.Output<string>;
  public readonly provider?: pulumi.ProviderResource;
  public readonly secretProvider?: pulumi.ProviderResource;
  public readonly appName: AN;

  constructor(
    name: NoUnion<AN>,
    args: AppConfigs<AN, DBT, NS>
    // opts: pulumi.ComponentResourceOptions
  ) {
    super("k8sjs:service:ServiceDeployment", name, {} /* opts */);
    // const provider = opts?.provider;
    this.appName = name;
    const { envVars, kubeConfig } = args;
    const metadata = {
      ...args.metadata,
      // ...getArgoAppSyncWaveAnnotation("service")
    };
    const resourceName = metadata.name;

    this.provider = new k8s.Provider(
      this.appName,
      {
        renderYamlToDirectory: this.getServiceDir(),
      },
      { parent: this }
    );
    // this.provider = this.getProvider();

    this.configMaps = new kx.ConfigMap(
      `${resourceName}-configmap`,
      {
        data: { config: "very important data" },
        metadata,
      },
      { provider: this.getProvider(), parent: this }
    );

    const secrets = getSecretsForApp(this.appName, ENVIRONMENT);
    // Create a Kubernetes Secret.
    this.secret = new kx.Secret(
      `${resourceName}-secret`,
      {
        stringData: {
          //  password: "fakepassword",
          ...secrets,
        },
        metadata: {
          ...metadata,
          annotations: {
            "sealedsecrets.bitnami.com/managed": "true",
            ...APPLICATION_AUTOMERGE_ANNOTATION
          }
        },
      },
      // //TODO: Confirm why secret has a separate provider
      // { provider: this.secretProvider, parent: this }
      { provider: this.getProvider(), parent: this }
    );

    // Define a Pod.
    const podBuilder = new kx.PodBuilder({
      initContainers: [],
      containers: [
        {
          env: {
            // CONFIG: this.configMaps.asEnvValue("config"),
            // PASSWORD: this.secret.asEnvValue("password"),
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
            "linkerd.io/inject": "enabled",
          },
        },
      },
      { provider: this.getProvider(), parent: this }
    );

    this.service = this.deployment.createService({
      type: kx.types.ServiceType.ClusterIP,
      ports: [
        {
          port: Number(envVars.APP_PORT),
          protocol: "TCP",
          name: `${resourceName}-http`,
          targetPort: Number(envVars.APP_PORT),
        },
      ],
    });

    this.argocdApplication = createArgocdChildrenApplication({
      namespace: metadata.namespace,
      // resourceType: "services",
      resourceName: this.appName,
      opts: {
        parent: this,
      },
    });

    const useLoadBalancer = new pulumi.Config("useLoadBalancer") ?? false;
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
    const secretObject = getSecretsForApp(this.appName, ENVIRONMENT);
    const keyValueEntries = Object.keys(secretObject).map((key) => [
      key,
      secretInstance.asEnvValue(key),
    ]);
    return Object.fromEntries(keyValueEntries);
  };

  getProvider = () => {
    return this.provider;
  };

  getServiceDir = (): string => {
    return getPathToResource({
      resourceType: "services",
      environment: ENVIRONMENT,
      resourceName: this.appName,
    });
  };
}

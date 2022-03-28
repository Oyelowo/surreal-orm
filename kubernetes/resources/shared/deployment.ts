import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";
import { getSecretForApp } from "./helpers";
import { NoUnion } from "./types/own-types";
import {
  AppConfigs,
  AppName,
  DBType,
  NamespaceOfApps,
} from "./types/own-types";

import * as z from "zod";
const secretsSchema = z.object({
  USERNAME: z.string().nonempty(),
  PASSWORD: z.string().nonempty(),
});

type MySecret = z.infer<typeof secretsSchema>;

export class ServiceDeployment<
  AN extends AppName,
  DBT extends DBType,
  NS extends NamespaceOfApps
> extends pulumi.ComponentResource {
  public readonly deployment: kx.Deployment;
  public readonly configMaps: kx.ConfigMap;
  public readonly secret: kx.Secret;
  public readonly service: kx.Service;
  public readonly ipAddress?: pulumi.Output<string>;
  public readonly secretProvider?: pulumi.ProviderResource;
  public readonly appName: AN;

  constructor(
    name: NoUnion<AN>,
    args: AppConfigs<AN, DBT, NS>,
    opts: pulumi.ComponentResourceOptions
  ) {
    super("k8sjs:service:ServiceDeployment", name, {}, opts);
    const provider = opts?.provider;
    this.appName = name;
    const { envVars, kubeConfig, metadata } = args;
    const resourceName = metadata.name;

    this.configMaps = new kx.ConfigMap(
      `${resourceName}-configmap`,
      {
        data: { config: "very important data" },
        metadata,
      },
      { provider, parent: this }
    );

    const secrets = getSecretForApp<AN>(this.appName);
    // Create a Kubernetes Secret.
    this.secret = new kx.Secret(
      `${resourceName}-secret`,
      {
        stringData: {
             password: "fakepassword",
          ...secrets,
        },
        metadata,
      },
      { provider: this.secretProvider, parent: this }
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
    });

    // Create a Kubernetes Deployment.
    this.deployment = new kx.Deployment(
      `${resourceName}-deployment`,
      {
        spec: podBuilder.asDeploymentSpec({
          replicas: kubeConfig.replicaCount,
        }),
        metadata,
      },
      { provider, parent: this }
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

    const useLoadBalancer = new pulumi.Config("useLoadBalancer") ?? false;
    if (useLoadBalancer) {
      this.ipAddress = this.service.status.loadBalancer.ingress[0].ip;
    } else {
      this.ipAddress = this.service.spec.clusterIP;
    }
  }

  /** 
     Maps secret object to what kx can understand to produce secretRef automagically
   */
  #secretsObjectToEnv = (secretInstance: kx.Secret) => {
    const secretObject = getSecretForApp(this.appName);
    const keyValueEntries = Object.keys(secretObject).map((key) => [
      key,
      secretInstance.asEnvValue(key),
    ]);
    return Object.fromEntries(keyValueEntries);
  };
}

import * as kx from '@pulumi/kubernetesx';
import * as pulumi from '@pulumi/pulumi';

import { NoUnion } from '../shared/types';
import { AppConfigs, AppName, DBType, NamespaceOfApps } from './types';

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

  constructor(
    name: NoUnion<AN>,
    args: AppConfigs<AN, DBT, NS>,
    opts: pulumi.ComponentResourceOptions
  ) {
    super("k8sjs:service:ServiceDeployment", name, {}, opts);
    const provider = opts?.provider;

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

    // Create a Kubernetes Secret.
    this.secret = new kx.Secret(
      `${resourceName}-secret`,
      {
        stringData: {
          password: "fakepassword",
        },
        metadata,
      },
      { provider, parent: this }
    );

    // Define a Pod.
    const podBuilder = new kx.PodBuilder({
      initContainers: [],
      containers: [
        {
          env: {
            CONFIG: this.configMaps.asEnvValue("config"),
            PASSWORD: this.secret.asEnvValue("password"),
            ...envVars,
          },
          image: kubeConfig.image,
          ports: { http: 8000 },
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
        spec: podBuilder.asDeploymentSpec({ replicas: 3 }),
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
          targetPort: 8000,
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
}

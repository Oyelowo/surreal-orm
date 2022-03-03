import { RecursivePartial, Settings } from "../shared/types";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "../shared/cluster";
import { devNamespace, devNamespaceName } from "../shared/namespaces";
import * as random from "@pulumi/random";
import * as pulumi from "@pulumi/pulumi";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import { graphqlPostgresSettings } from "./settings";
// Prefix by the name of deployment to make them unique across stack

// const { resourceName } = graphqlPostgresSettings;
const { kubeConfig, envVars } = graphqlPostgresSettings;
const resourceName = kubeConfig.resourceName;

const metadataObject = {
  metadata: {
    name: resourceName,
    namespace: devNamespaceName,
  },
};

// Create a Kubernetes ConfigMap.
export const graphqlPostgresConfigMap = new kx.ConfigMap(
  `${resourceName}-configmap`,
  {
    data: { config: "very important data" },
    ...metadataObject,
  },
  { provider }
);

// Create a Kubernetes Secret.
export const graphqlPostgresSecret = new kx.Secret(
  `${resourceName}-secret`,
  {
    stringData: {
      password: "fakepassword",
    },
    ...metadataObject,
  },
  { provider }
);

// Define a Pod.
const graphqlPostgresPodBuilder = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: graphqlPostgresConfigMap.asEnvValue("config"),
        PASSWORD: graphqlPostgresSecret.asEnvValue("password"),
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
export const graphqlPostgresDeployment = new kx.Deployment(
  `${resourceName}-deployment`,
  {
    spec: graphqlPostgresPodBuilder.asDeploymentSpec({ replicas: 3 }),
    ...metadataObject,
  },
  { provider }
);

export const graphqlPostgresService = graphqlPostgresDeployment.createService({
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
export let graphqlPostgresAppIp: pulumi.Output<string>;
if (useLoadBalancer) {
  graphqlPostgresAppIp =
    graphqlPostgresService.status.loadBalancer.ingress[0].ip;
} else {
  graphqlPostgresAppIp = graphqlPostgresService.spec.clusterIP;
}

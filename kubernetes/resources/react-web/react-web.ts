import { Settings } from "../shared/types";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "../shared/cluster";
import { reactWebSettings } from "./settings";
import { devNamespaceName } from "../shared/namespaces";

const { envVars, kubeConfig } = reactWebSettings;

const resourceName = kubeConfig.resourceName;

const metadataObject = {
  metadata: {
    name: resourceName,
    namespace: devNamespaceName,
  },
} as const;

// Create a Kubernetes ConfigMap.
export const reactWebConfigMap = new kx.ConfigMap(
  `${resourceName}-configmap`,
  {
    ...metadataObject,
    data: { config: "very important data" },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const reactWebSecret = new kx.Secret(
  `${resourceName}-secret`,
  {
    ...metadataObject,
    stringData: {
      password: "very-weak-password",
    },
  },
  { provider }
);

// Define a Pod.
export const reactWebPodBuilder = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: reactWebConfigMap.asEnvValue("config"),
        PASSWORD: reactWebSecret.asEnvValue("password"),
        ...envVars,
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
export const reactWebDeployment = new kx.Deployment(
  `${resourceName}-deployment`,
  {
    ...metadataObject,
    spec: reactWebPodBuilder.asDeploymentSpec({ replicas: 2 }),
  },
  { provider }
);

// // Create a Kubernetes Service.
export const reactWebService = reactWebDeployment.createService({
  type: kx.types.ServiceType.ClusterIP,
});

console.log("rerekrekrek", reactWebService.urn);

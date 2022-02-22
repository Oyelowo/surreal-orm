import { Settings } from "../shared/types";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "../shared/cluster";
import { reactWebEnvVars } from "./settings";
import { devNamespaceName } from "../shared/namespaces";

export const reactWebSettings: Settings = {
  requestMemory: "100Mi",
  requestCpu: "100m",
  limitMemory: "250Mi",
  limitCpu: "250m",
  host: "0.0.0.0",
  resourceName: "react-web",
  image: "oyelowo/react-web",
};

const { resourceName } = reactWebSettings;

const metadataObject = {
  metadata: {
    name: resourceName,
    namespace: devNamespaceName,
  },
};

// Create a Kubernetes ConfigMap.
export const reactWebConfigMap = new kx.ConfigMap(
  "react-web-configmap",
  {
    ...metadataObject,
    data: { config: "very important data" },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const reactWebSecret = new kx.Secret(
  "react-web-secret",
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
        HOST: "",
        PORT: "",
      },
      image: reactWebSettings.image,
      ports: { http: Number(reactWebEnvVars.APP_PORT) },
      volumeMounts: [],
      resources: {
        limits: {
          memory: reactWebSettings.limitMemory,
          cpu: reactWebSettings.limitCpu,
        },
        requests: {
          memory: reactWebSettings.requestMemory,
          cpu: reactWebSettings.requestCpu,
        },
      },
    },
  ],
});

// Create a Kubernetes Deployment.
export const reactWebDeployment = new kx.Deployment(
  "react-web-deployment",
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

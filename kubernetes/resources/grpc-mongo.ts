import { Settings } from "./shared/types";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "./shared/cluster";

// Prefix by the name of deployment to make them unique across stack

const grpcMongoSettings: Settings = {
  resourceName: "graphql-mongo",
  image: "oyelowo/graphql-mongo",
  requestMemory: "1G",
  requestCpu: "100m",
  limitMemory: "1G",
  limitCpu: "100m",
  host: "0.0.0.0",
};

const { resourceName, image } = grpcMongoSettings;

// Create a Kubernetes ConfigMap.
export const grpcMongoConfigMap = new kx.ConfigMap(
  `${resourceName}-configMap`,
  {
    data: { config: "very important data" },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const grpcMongoSecret = new kx.Secret(
  `${resourceName}-secret`,
  {
    stringData: {
      password: "very-weak-password",
    },
  },
  { provider }
);

// Define a Pod.
export const grpcMongoPodBuilder = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: grpcMongoConfigMap.asEnvValue("config"),
        PASSWORD: grpcMongoSecret.asEnvValue("password"),
        HOST: "",
        PORT: "",
      },
      image,
      ports: { http: 8080 },
      volumeMounts: [],
      resources: {
        limits: {
          memory: grpcMongoSettings.limitMemory,
          cpu: grpcMongoSettings.limitCpu,
        },
        requests: {
          memory: grpcMongoSettings.requestMemory,
          cpu: grpcMongoSettings.requestCpu,
        },
      },
    },
  ],
});

// Create a Kubernetes Deployment.
export const grpcMongoDeployment = new kx.Deployment(
  `${resourceName}-deployment`,
  {
    spec: grpcMongoPodBuilder.asDeploymentSpec({ replicas: 3 }),
  },
  { provider }
);

// // Create a Kubernetes Service.
export const grpcMongoService = grpcMongoDeployment.createService({
  type: kx.types.ServiceType.ClusterIP,
});

console.log("rerekrekrek", grpcMongoService.urn);

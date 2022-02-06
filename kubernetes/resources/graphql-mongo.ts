import { Settings } from './types';
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "./cluster";

// Prefix by the name of deployment to make them unique across stack

export const graphqlMongoSettings: Settings = {
  requestMemory: "1G",
  requestCpu: "100m",
  limitMemory: "1G",
  limitCpu: "100m",
  host: "0.0.0.0",
};

// Create a Kubernetes ConfigMap.
export const graphqlMongoConfigMap = new kx.ConfigMap(
  "graphqlMongoConfigMap",
  {
    data: { config: "very important data" },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const graphqlMongoSecret = new kx.Secret(
  "graphqlMongoSecret",
  {
    stringData: {
      password: "very-weak-password",
    },
  },
  { provider }
);

// Define a Pod.
export const graphqlMongoPodBuilder = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: graphqlMongoConfigMap.asEnvValue("config"),
        PASSWORD: graphqlMongoSecret.asEnvValue("password"),
        HOST: "",
        PORT: "",
      },
      image: "graphql-mongo",
      ports: { http: 8080 },
      volumeMounts: [],
      resources: {
        limits: {
          memory: graphqlMongoSettings.limitMemory,
          cpu: graphqlMongoSettings.limitCpu,
        },
        requests: {
          memory: graphqlMongoSettings.requestMemory,
          cpu: graphqlMongoSettings.requestCpu,
        },
      },
    },
  ],
});

// Create a Kubernetes Deployment.
export const graphqlMongoDeployment = new kx.Deployment(
  "graphqlMongoDeployment",
  {
    spec: graphqlMongoPodBuilder.asDeploymentSpec({ replicas: 3 }),
  },
  { provider }
);

// // Create a Kubernetes Service.
export const graphqlMongoService = graphqlMongoDeployment.createService({
  type: kx.types.ServiceType.ClusterIP,
});

console.log("rerekrekrek", graphqlMongoService.urn);

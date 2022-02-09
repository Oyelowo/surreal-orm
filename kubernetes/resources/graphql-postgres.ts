import { Settings } from './types';
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "./cluster";

// Prefix by the name of deployment to make them unique across stack

export const graphqlPostgresSettings: Settings = {
  requestMemory: "1G",
  requestCpu: "100m",
  limitMemory: "1G",
  limitCpu: "100m",
  host: "0.0.0.0",
};

// Create a Kubernetes ConfigMap.
export const graphqlPostgresConfigMap = new kx.ConfigMap(
  "graphql-postgres-configMap",
  {
    data: { config: "very important data" },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const graphqlPostgresSecret = new kx.Secret(
  "graphql-postgres-secret",
  {
    stringData: {
      password: "very-weak-password",
    },
  },
  { provider }
);

// Define a Pod.
export const graphqlPostgresPodBuilder = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: graphqlPostgresConfigMap.asEnvValue("config"),
        PASSWORD: graphqlPostgresSecret.asEnvValue("password"),
        HOST: "",
        PORT: "",
      },
      image: "graphql-postgres",
      ports: { http: 8080 },
      volumeMounts: [],
      resources: {
        limits: {
          memory: graphqlPostgresSettings.limitMemory,
          cpu: graphqlPostgresSettings.limitCpu,
        },
        requests: {
          memory: graphqlPostgresSettings.requestMemory,
          cpu: graphqlPostgresSettings.requestCpu,
        },
      },
    },
  ],
});

// Create a Kubernetes Deployment.
export const graphqlPostgresDeployment = new kx.Deployment(
  "graphql-postgres-deployment",
  {
    spec: graphqlPostgresPodBuilder.asDeploymentSpec({ replicas: 3 }),
  },
  { provider }
);

// // Create a Kubernetes Service.
export const graphqlPostgresService = graphqlPostgresDeployment.createService({
  type: kx.types.ServiceType.ClusterIP,
});

console.log("rerekrekrek", graphqlPostgresService.urn);

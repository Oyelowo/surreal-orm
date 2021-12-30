import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "./shared";

type Memory = `${number}${
  | "E"
  | "P"
  | "T"
  | "G"
  | "M"
  | "k"
  | "m"
  | "Ei"
  | "Pi"
  | "Ti"
  | "Gi"
  | "Mi"
  | "Ki"}`;

type CPU = `${number}${"m"}`;

interface Settings {
  requestMemory: Memory;
  requestCpu: CPU;
  limitMemory: Memory;
  limitCpu: CPU;
  host: string;
}

export const backendMainSettings: Settings = {
  requestMemory: "1G",
  requestCpu: "100m",
  limitMemory: "1G",
  limitCpu: "100m",
  host: "0.0.0.0",
};

// Create a Kubernetes ConfigMap.
export const configMap = new kx.ConfigMap(
  "configMap",
  {
    data: { config: "very important data" },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const secret = new kx.Secret(
  "secret",
  {
    stringData: {
      password: "very-weak-password",
    },
  },
  { provider }
);


// Define a Pod.
export const pb = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: configMap.asEnvValue("config"),
        PASSWORD: secret.asEnvValue("password"),
        HOST: "",
        PORT: "",
      },
      image: "nginx",
      ports: { http: 8080 },
      volumeMounts: [],
      resources: {
        limits: {
          memory: backendMainSettings.limitMemory,
          cpu: backendMainSettings.limitCpu,
        },
        requests: {
          memory: backendMainSettings.requestMemory,
          cpu: backendMainSettings.requestCpu,
        },
      },
    },
  ],
});

// Create a Kubernetes Deployment.
export const deployment = new kx.Deployment(
  "nginx",
  {
    spec: pb.asDeploymentSpec({ replicas: 3 }),
  },
  { provider }
);

// // Create a Kubernetes Service.
export const service = deployment.createService({
    type: kx.types.ServiceType.ClusterIP,
});


console.log("rerekrekrek",service.urn)
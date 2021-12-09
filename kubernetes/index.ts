import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";

// Instantiate a Kubernetes Provider and specify the render directory.
const provider = new k8s.Provider("render-yaml", {
  renderYamlToDirectory: "rendered",
});

// Create a Kubernetes PersistentVolumeClaim.
const pvc = new kx.PersistentVolumeClaim(
  "data",
  {
    spec: {
      accessModes: ["ReadWriteOnce"],
      resources: { requests: { storage: "1Gi" } },
    },
  },
  { provider }
);

// Create a Kubernetes ConfigMap.
const cm = new kx.ConfigMap(
  "cm",
  {
    data: { config: "very important data" },
  },
  { provider }
);

// Create a Kubernetes Secret.
const secret = new kx.Secret(
  "secret",
  {
    stringData: {
      password: "very-weak-password",
    },
  },
  { provider }
);

// Define a Pod.
const pb = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: cm.asEnvValue("config"),
        PASSWORD: secret.asEnvValue("password"),
      },
      image: "nginx",
      ports: { http: 8080 },
      volumeMounts: [pvc.mount("/data")],
      resources: {
        limits: {
          memory: "1G",
          cpu: "30cu",
        },
        requests: {
          memory: "500",
          cpu: "12cu",
        },
      },
    },
  ],
});

// Create a Kubernetes Deployment.
const deployment = new kx.Deployment(
  "nginx",
  {
    spec: pb.asDeploymentSpec({ replicas: 3 }),
  },
  { provider }
);

// Create a Kubernetes StatefulSet.
const statefulSet = new kx.StatefulSet(
  "postgres",
  {
    spec: pb.asStatefulSetSpec({ replicas: 3 }),
  },
  { provider }
);


const providerHelm = new k8s.Provider("render-yaml2", {
  renderYamlToDirectory: "renderedHelmChart",
});

// Deploy Wordpress into our cluster.
const wordpress = new k8s.helm.v2.Chart(
  "postgres",
  {
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
      chart: "postgresql-ha",
      namespace: "data-space",
    values: {
      wordpressBlogName: "My Cool Kubernetes Blog!",
    },
  },
  { providers: { kubernetes: providerHelm } }
);

// // Create a Kubernetes Service.
// const service = deployment.createService({
//     type: kx.types.ServiceType.LoadBalancer,
// });

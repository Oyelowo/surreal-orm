import { MongodbHelmValuesBitnami } from "./shared/MongodbBitnami";
import { RecursivePartial, Settings } from "./shared/types";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "./shared/cluster";
import { devNamespace, devNamespaceName } from "./shared/namespaces";
import * as random from "@pulumi/random";
import * as pulumi from "@pulumi/pulumi";
import { Namespace } from "@pulumi/kubernetes/core/v1";
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
  "graphql-mongo-configMap",
  {
    data: { config: "very important data" },
    metadata: {
      namespace: devNamespaceName,
    },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const graphqlMongoSecret = new kx.Secret(
  "graphql-mongo-secret",
  {
    stringData: {
      password: "very-weak-password",
    },
    metadata: {
      namespace: devNamespaceName,
    },
  },
  { provider }
);

const graphqlMongoEnvironmentVariables = {
  APP_ENVIRONMENT: "local",
  APP_HOST: "0.0.0.0",
  APP_PORT: "8000",
  MONGODB_NAME: "mydb",
  MONGODB_USERNAME: "mongo",
  MONGODB_PASSWORD: "fakepassword",
  MONGODB_HOST: "mongodb-graphql",
  MONGODB_PORT: "27017",
};

// Define a Pod.
export const graphqlMongoPodBuilder = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: graphqlMongoConfigMap.asEnvValue("config"),
        PASSWORD: graphqlMongoSecret.asEnvValue("password"),
        ...graphqlMongoEnvironmentVariables,
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

// export const graphqlMongoNamespace = new Namespace("na", {metadata: {name: "na"}}, {provider});
// Create a Kubernetes Deployment.
export const graphqlMongoDeployment = new kx.Deployment(
  "graphql-mongo-deployment",
  {
    spec: graphqlMongoPodBuilder.asDeploymentSpec({ replicas: 3 }),
    metadata: {
      namespace: devNamespaceName,
    },
  },
  { provider }
);

// // Create a Kubernetes Service.
export const graphqlMongoService = graphqlMongoDeployment.createService({
  type: kx.types.ServiceType.ClusterIP,
  ports: [
    {
      port: Number(graphqlMongoEnvironmentVariables.APP_PORT),
      protocol: "TCP",
      name: "graphql-mongo-http",
      //targetPort: 434,
    },
  ],
});

/* MONGODB STATEFULSET */
const mongoValues: RecursivePartial<MongodbHelmValuesBitnami> = {
  useStatefulSet: true,
  architecture: "replicaset",
  replicaCount: 3,
  global: {
    namespaceOverride: devNamespaceName,
  },
};

export const graphqlMongoMongodb = new k8s.helm.v3.Chart(
  "mongodb-helm",
  {
    chart: "mongodb",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "11.0.0",
    values: mongoValues,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider }
);

// Export the public IP for WordPress.
// const frontend2 = mongodb2.getResource("v1/Service", "mongodbdev-mongodb");
// export const frontendIp2 = frontend2.status.loadBalancer.ingress[0].ip;

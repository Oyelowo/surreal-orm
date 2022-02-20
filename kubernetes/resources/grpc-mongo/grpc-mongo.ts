import { MongodbHelmValuesBitnami } from "../shared/MongodbHelmValuesBitnami";
import { RecursivePartial, Settings } from "../shared/types";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "../shared/cluster";
import { devNamespace, devNamespaceName } from "../shared/namespaces";
import * as random from "@pulumi/random";
import * as pulumi from "@pulumi/pulumi";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import { grpcMongoEnvVars, grpcMongoSettings } from "./settings";
// Prefix by the name of deployment to make them unique across stack

const { resourceName } = grpcMongoSettings;

const metadataObject = {
  metadata: {
    name: resourceName,
    namespace: devNamespaceName,
  },
};

// Create a Kubernetes ConfigMap.
export const grpcMongoConfigMap = new kx.ConfigMap(
  `${resourceName}-configmap`,
  {
    data: { config: "very important data" },
    ...metadataObject,
  },
  { provider }
);

// Create a Kubernetes Secret.
export const grpcMongoSecret = new kx.Secret(
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
export const grpcMongoPodBuilder = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: grpcMongoConfigMap.asEnvValue("config"),
        PASSWORD: grpcMongoSecret.asEnvValue("password"),
        ...grpcMongoEnvVars,
      },
      image: grpcMongoSettings.image,
      ports: { http: 8000 },
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
    ...metadataObject,
  },
  { provider }
);

// Create a Kubernetes Service.
// export const grpcMongoService2 = new kx.Service(
//   `${resourceName}-service`,
//   {
//     metadata: {name: resourceName, namespace: devNamespaceName},
//     spec: {
//       type: kx.types.ServiceType.ClusterIP,
//       ports: [
//         {
//           port: Number(grpcMongoEnvironmentVariables.APP_PORT),
//           targetPort: Number(grpcMongoEnvironmentVariables.APP_PORT),
//           protocol: "TCP",
//           name: `${resourceName}-http`,
//           // targetPort: 434,
//         },
//       ],
//       selector: {}
//     },
//   }
//   , {provider}
// );
// import { generateService } from "../shared/helpers";
// export const grpcMongoService2 = generateService({
//   serviceName: `${resourceName}-service`,
//   deployment: grpcMongoDeployment,
//   args: {
//     type: kx.types.ServiceType.ClusterIP,
//     // name: `${resourceName}-service`,
//     ports: [
//       {
//         port: Number(grpcMongoEnvironmentVariables.APP_PORT),
//         protocol: "TCP",
//         name: `${resourceName}-http`,
//         // targetPort: 434,
//       },
//     ],
//   },
// });

export const grpcMongoService = grpcMongoDeployment.createService({
  type: kx.types.ServiceType.ClusterIP,
  ports: [
    {
      port: Number(grpcMongoEnvVars.APP_PORT),
      protocol: "TCP",
      name: `${resourceName}-http`,
      targetPort: 8000,
    },
  ],
});

// Export the public IP for WordPress.
// const frontend2 = mongodb2.getResource("v1/Service", "mongodbdev-mongodb");
// export const frontendIp2 = frontend2.status.loadBalancer.ingress[0].ip;

// const frontend2 = grpcMongoService.spec.clusterIP;
// Export the frontend IP.
const useLoadBalancer = new pulumi.Config("useLoadBalancer") ?? false;
export let grpcMongoAppIp: pulumi.Output<string>;
if (useLoadBalancer) {
  grpcMongoAppIp = grpcMongoService.status.loadBalancer.ingress[0].ip;
} else {
  grpcMongoAppIp = grpcMongoService.spec.clusterIP;
}

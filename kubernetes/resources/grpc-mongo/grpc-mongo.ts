import { MongodbHelmValuesBitnami } from "../shared/MongodbHelmValuesBitnami";
import { RecursivePartial, Settings } from "../shared/types";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { provider } from "../shared/cluster";
import { devNamespace, devNamespaceName } from "../shared/namespaces";
import * as random from "@pulumi/random";
import * as pulumi from "@pulumi/pulumi";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import { grpcMongoSettings } from "./settings";
// Prefix by the name of deployment to make them unique across stack

const { envVars, kubeConfig, metadata } = grpcMongoSettings;
const resourceName = kubeConfig.resourceName;

// Create a Kubernetes ConfigMap.
export const grpcMongoConfigMap = new kx.ConfigMap(
  `${resourceName}-configmap`,
  {
    data: { config: "very important data" },
    metadata,
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
    metadata,
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
export const grpcMongoDeployment = new kx.Deployment(
  `${resourceName}-deployment`,
  {
    spec: grpcMongoPodBuilder.asDeploymentSpec({ replicas: 3 }),
    metadata,
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
      port: Number(envVars.APP_PORT),
      protocol: "TCP",
      name: `${resourceName}-http`,
      targetPort: Number(envVars.APP_PORT),
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

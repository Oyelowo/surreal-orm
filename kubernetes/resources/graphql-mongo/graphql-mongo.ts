import * as k8s from '@pulumi/kubernetes';
import { Namespace } from '@pulumi/kubernetes/core/v1';
import * as kx from '@pulumi/kubernetesx';
import * as pulumi from '@pulumi/pulumi';
import * as random from '@pulumi/random';

import { provider } from '../shared/cluster';
import { MongodbHelmValuesBitnami } from '../shared/MongodbHelmValuesBitnami';
import { devNamespace, devNamespaceName } from '../shared/namespaces';
import { RecursivePartial, Settings } from '../shared/types';
import { graphqlMongoSettings } from './settings';

// Prefix by the name of deployment to make them unique across stack

const { kubeConfig, envVars } = graphqlMongoSettings;
const resourceName = kubeConfig.resourceName;

// Create a Kubernetes ConfigMap.F
export const graphqlMongoConfigMap = new kx.ConfigMap(
  `${resourceName}-configmap`,
  {
    data: { config: "very important data" },
    metadata: {
      name: resourceName,
      namespace: devNamespaceName,
    },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const graphqlMongoSecret = new kx.Secret(
  `${resourceName}-secret`,
  {
    stringData: {
      password: "fakepassword",
    },
    metadata: {
      name: resourceName,
      namespace: devNamespaceName,
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
        ...envVars,
      },
      image: kubeConfig.image,
      ports: { http: 8000 },
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
export const graphqlMongoDeployment = new kx.Deployment(
  `${resourceName}-deployment`,
  {
    spec: graphqlMongoPodBuilder.asDeploymentSpec({ replicas: 3 }),
    metadata: {
      name: resourceName,
      namespace: devNamespaceName,
    },
  },
  { provider }
);

// Create a Kubernetes Service.
// export const graphqlMongoService2 = new kx.Service(
//   `${resourceName}-service`,
//   {
//     metadata: {name: resourceName, namespace: devNamespaceName},
//     spec: {
//       type: kx.types.ServiceType.ClusterIP,
//       ports: [
//         {
//           port: Number(graphqlMongoEnvironmentVariables.APP_PORT),
//           targetPort: Number(graphqlMongoEnvironmentVariables.APP_PORT),
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
// export const graphqlMongoService2 = generateService({
//   serviceName: `${resourceName}-service`,
//   deployment: graphqlMongoDeployment,
//   args: {
//     type: kx.types.ServiceType.ClusterIP,
//     // name: `${resourceName}-service`,
//     ports: [
//       {
//         port: Number(graphqlMongoEnvironmentVariables.APP_PORT),
//         protocol: "TCP",
//         name: `${resourceName}-http`,
//         // targetPort: 434,
//       },
//     ],
//   },
// });

export const graphqlMongoService = graphqlMongoDeployment.createService({
  type: kx.types.ServiceType.ClusterIP,
  ports: [
    {
      port: Number(envVars.APP_PORT),
      protocol: "TCP",
      name: `${resourceName}-http`,
      targetPort: 8000,
    },
  ],
});

// Export the public IP for WordPress.
// const frontend2 = mongodb2.getResource("v1/Service", "mongodbdev-mongodb");
// export const frontendIp2 = frontend2.status.loadBalancer.ingress[0].ip;

// const frontend2 = graphqlMongoService.spec.clusterIP;
// Export the frontend IP.
const useLoadBalancer = new pulumi.Config("useLoadBalancer") ?? false;
export let graphqlMongoAppIp: pulumi.Output<string>;
if (useLoadBalancer) {
  graphqlMongoAppIp = graphqlMongoService.status.loadBalancer.ingress[0].ip;
} else {
  graphqlMongoAppIp = graphqlMongoService.spec.clusterIP;
}

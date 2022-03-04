import { AppConfigs, AppName, DBType, NamespaceOfApps } from "./types";
import * as k8s from "@pulumi/kubernetes";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";
import * as random from "@pulumi/random";

import { provider } from "../shared/cluster";
import { MongodbHelmValuesBitnami } from "../shared/MongodbHelmValuesBitnami";
import { devNamespace, devNamespaceName } from "../shared/namespaces";
import { RecursivePartial, Settings } from "../shared/types";

// Prefix by the name of deployment to make them unique across stack

// const { kubeConfig, envVars, metadata } = graphqlMongoSettings;
// const resourceName = kubeConfig.resourceName;

function getDeploymentService<
  AN extends AppName,
  DBT extends DBType,
  NS extends NamespaceOfApps
>(appConfig: AppConfigs<AN, DBT, NS>) {
  const { envVars, kubeConfig, metadata } = appConfig;
  const resourceName = kubeConfig.resourceName;
  // Create a Kubernetes ConfigMap.F
  const graphqlMongoConfigMap = new kx.ConfigMap(
    `${resourceName}-configmap`,
    {
      data: { config: "very important data" },
      metadata,
    },
    { provider }
  );

  // Create a Kubernetes Secret.
  const graphqlMongoSecret = new kx.Secret(
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
  const graphqlMongoPodBuilder = new kx.PodBuilder({
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
  const graphqlMongoDeployment = new kx.Deployment(
    `${resourceName}-deployment`,
    {
      spec: graphqlMongoPodBuilder.asDeploymentSpec({ replicas: 3 }),
      metadata,
    },
    { provider }
  );

  // Create a Kubernetes Service.
  // const graphqlMongoService2 = new kx.Service(
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
  // const graphqlMongoService2 = generateService({
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

  const graphqlMongoService = graphqlMongoDeployment.createService({
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

  // the public IP for WordPress.
  // const frontend2 = mongodb2.getResource("v1/Service", "mongodbdev-mongodb");
  // const frontendIp2 = frontend2.status.loadBalancer.ingress[0].ip;

  // const frontend2 = graphqlMongoService.spec.clusterIP;
  // the frontend IP.
  const useLoadBalancer = new pulumi.Config("useLoadBalancer") ?? false;
  let graphqlMongoAppIp: pulumi.Output<string>;
  if (useLoadBalancer) {
    graphqlMongoAppIp = graphqlMongoService.status.loadBalancer.ingress[0].ip;
  } else {
    graphqlMongoAppIp = graphqlMongoService.spec.clusterIP;
  }

  return {
    graphqlMongoDeployment,
  };
}

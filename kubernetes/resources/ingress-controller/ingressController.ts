import { getEnvironmentVariables } from "./../shared/validations";
// import { clusterSetupDirectory } from "../shared/manifestsDirectory";
import * as k8s from "@pulumi/kubernetes";

import { graphqlMongoSettings } from "../graphql-mongo/settings";
import { reactWebSettings } from "../react-web/settings";
// import { applicationsDirectory } from "../shared/manifestsDirectory";
import { IngressControllerValuesBitnami } from "../shared/types/helm-charts/ingressControllerValuesBitnami";
import { namespaceNames } from "../shared/namespaces";
import { NginxConfiguration } from "../shared/types/nginxConfigurations";
import { RecursivePartial } from "../shared/types/own-types";
import { getPathToNonApplicationDir } from "../shared/manifestsDirectory";

export const ingressControllerName = "nginx-ingress-controller";

export const ingressControllerDirName = getPathToNonApplicationDir(
  ingressControllerName,
  getEnvironmentVariables().ENVIRONMENT
);

export const ingressControllerProvider = new k8s.Provider(ingressControllerDirName, {
  renderYamlToDirectory: ingressControllerDirName,
});

// Install the NGINX ingress controller to our cluster. The controller
// consists of a Pod and a Service. Install it and configure the controller
// to publish the load balancer IP address on each Ingress so that
// applications can depend on the IP address of the load balancer if needed.
// export const ingressNginx = new nginx.IngressController(
//   "nginx-ingress-controller",
//   {
//     controller: {
//       publishService: {
//         enabled: true,
//       },
//     },
//   },
//   { provider: provider }
// );

const ingressControllerValues: RecursivePartial<IngressControllerValuesBitnami> = {
  // containerPorts: {
  //   http: 8000,
  //   https: 443,
  // },
  fullnameOverride: ingressControllerName,
};
// nginx-ingress-controller
// K3s also comes with a traefik ingress controoler. Disable that if using this
export const ingressNginxController = new k8s.helm.v3.Chart(
  ingressControllerName,
  {
    chart: "nginx-ingress-controller",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "9.1.8",
    values: ingressControllerValues,
    // namespace: "nginx-ingress",
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: ingressControllerProvider }
  // { provider }
);

type IngressClassName = "nginx" | "traefik";
const ingressClassName: IngressClassName = "nginx";

const appBase = "oyelowo";
// // Next, expose the app using an Ingress.

const annotations: Partial<NginxConfiguration> = {
  "nginx.ingress.kubernetes.io/ssl-redirect": "false",
  "nginx.ingress.kubernetes.io/use-regex": "true",
};
export const appIngress = new k8s.networking.v1.Ingress(
  `${appBase}-ingress`,
  {
    metadata: {
      name: "nginx-ingress",
      namespace: namespaceNames.applications,
      annotations: annotations as any,
    },
    spec: {
      ingressClassName,
      rules: [
        {
          // Replace this with your own domain!
          // host: "myservicea.foo.org",
          // TODO: Change to proper domain name for prod and other environments in case of necessity
          host: "localhost",
          http: {
            paths: [
              {
                pathType: "Prefix",
                // path: "/?(.*)",
                path: "/",
                backend: {
                  service: {
                    name: reactWebSettings.metadata.name,
                    port: { number: Number(reactWebSettings.envVars.APP_PORT) },
                  },
                },
              },
              {
                pathType: "Prefix",
                path: "/graphql",
                backend: {
                  service: {
                    name: graphqlMongoSettings.metadata.name,
                    port: {
                      number: Number(graphqlMongoSettings.envVars.APP_PORT),
                    },
                  },
                },
              },
              // {
              //   pathType: "Prefix",
              //   path: "/graphql",
              //   backend: {
              //     service: {
              //       name: graphqlPostgresSettings.metadata.name,
              //       port: {
              //         number: Number(graphqlPostgresSettings.envVars.APP_PORT),
              //       },
              //     },
              //   },
              // },
            ],
          },
        },
        // {
        //   // Replace this with your own domain!
        //   host: "myserviceb.foo.org",
        //   http: {
        //     paths: [
        //       {
        //         pathType: "Prefix",
        //         path: "/",
        //         backend: {
        //           service: {
        //             name: graphqlPostgresSettings.resourceName,
        //             port: { number: Number(graphqlPostgresEnvVars.APP_PORT) },
        //           },
        //         },
        //       },
        //     ],
        //   },
        // },
      ],
    },
  },
  { provider: ingressControllerProvider }
);

// // export const appStatuses = apps;
// // export const controllerStatus = ctrl.status;

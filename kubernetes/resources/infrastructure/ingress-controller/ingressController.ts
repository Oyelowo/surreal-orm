import { getEnvironmentVariables } from "../../shared/validations";
// import { clusterSetupDirectory } from "../shared/manifestsDirectory";
import * as k8s from "@pulumi/kubernetes";

import { graphqlMongoSettings } from "../../services/graphql-mongo/settings";
import { reactWebSettings } from "../../services/react-web/settings";
// import { applicationsDirectory } from "../shared/manifestsDirectory";
import { IngressControllerValuesBitnami } from "../../shared/types/helm-charts/ingressControllerValuesBitnami";
import { namespaceNames } from "../../shared/namespaces";
import { NginxConfiguration } from "../../shared/types/nginxConfigurations";
import { RecursivePartial } from "../../shared/types/own-types";
import { getIngressControllerDir, ingressControllerName } from "../../shared/manifestsDirectory";
import { SECRET_NAME_NGINX } from "./certificate"
import { DNS_NAME_LINODE_BASE } from "./constant"
import { CLUSTER_ISSUER_NAME } from "../cert-manager";

const { ENVIRONMENT } = getEnvironmentVariables()
export const ingressControllerDir = getIngressControllerDir(ENVIRONMENT);

export const ingressControllerProvider = new k8s.Provider(ingressControllerDir, {
  renderYamlToDirectory: ingressControllerDir,
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
    version: "9.1.22",
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

type IngressAnootations = NginxConfiguration & { "cert-manager.io/cluster-issuer": typeof CLUSTER_ISSUER_NAME }
const annotations: Partial<IngressAnootations> = {
  "nginx.ingress.kubernetes.io/ssl-redirect": "false",
  "nginx.ingress.kubernetes.io/use-regex": "true",
  // "cert-manager.io/issuer": "letsencrypt-staging",
  "cert-manager.io/cluster-issuer": CLUSTER_ISSUER_NAME,
  // "kubernetes/io/ingress.class": "nginx"
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
      tls: [
        {
          // hosts: ["172.104.255.25"],
          hosts: [DNS_NAME_LINODE_BASE],
          // hosts: ["oyelowo.dev"],
          secretName: SECRET_NAME_NGINX
          // secretName: "oyelowo-tls"

        }
      ],
      rules: [
        {
          // Replace this with your own domain!
          // host: "myservicea.foo.org",
          // TODO: Change to proper domain name for prod and other environments in case of necessity
          host: ENVIRONMENT === "local" ? "localhost" : DNS_NAME_LINODE_BASE,
          // host: "172-104-255-25.ip.linodeusercontent.com",
          // host: ENVIRONMENT === "local" ? "localhost" : "172.104.255.25",
          // host: ENVIRONMENT === "local" ? "oyelowo.dev" : "oyelowo.dev",
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

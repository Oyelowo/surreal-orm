import { reactWebSettings, reactWebEnvVars } from "./../react-web/settings";
import { IngressControllerValuesBitnami } from "./../shared/ingressControllerValuesBitnami";
import { RecursivePartial } from "./../shared/types";
import { devNamespaceName } from "./../shared/namespaces";
import { provider } from "./../shared/cluster";
import {
  graphqlPostgresSettings,
  graphqlPostgresEnvVars,
} from "./../graphql-postgres/settings";
import {
  graphqlMongoSettings,
  graphqlMongoEnvVars,
} from "./../graphql-mongo/settings";
import * as k8s from "@pulumi/kubernetes";
import * as nginx from "@pulumi/kubernetes-ingress-nginx";
import { devNamespace } from "../shared/namespaces";

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
  containerPorts: {
    http: 8000,
    https: 443,
  },
};
// nginx-ingress-controller
// K3s also comes with a traefik ingress controoler. Disable that if using this
export const ingressNginx = new k8s.helm.v3.Chart(
  "nginx-ingress-controller-helm",
  {
    chart: "nginx-ingress-controller",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "9.1.8",
    values: ingressControllerValues,
    namespace: "default",
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider }
);

const appBase = "oyelowo";
// // Next, expose the app using an Ingress.
export const appIngress = new k8s.networking.v1.Ingress(
  `${appBase}-ingress`,
  {
    metadata: {
      name: "nginx-ingress",
      namespace: devNamespaceName,
      annotations: {
        // # Route all traffic to pod, but don't keep subpath (!)
        "nginx.ingress.kubernetes.io/rewrite-target": "/",
        // "kubernetes.io/ingress.class": "nginx",
        // "kubernetes.io/ingress.class": "traefik",
      },
    },
    spec: {
      rules: [
        {
          // Replace this with your own domain!
          // host: "myservicea.foo.org",
          host: "localhost",
          http: {
            paths: [
              {
                pathType: "Prefix",
                path: "/app",
                backend: {
                  service: {
                    name: reactWebSettings.resourceName,
                    port: { number: Number(reactWebEnvVars.APP_PORT) },
                  },
                },
              },
              {
                pathType: "Prefix",
                path: "/graphql",
                backend: {
                  service: {
                    name: graphqlMongoSettings.resourceName,
                    port: { number: Number(graphqlMongoEnvVars.APP_PORT) },
                  },
                },
              },
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
  { provider: provider }
);

// // export const appStatuses = apps;
// // export const controllerStatus = ctrl.status;

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

// Install the NGINX ingress controller to our cluster. The controller
// consists of a Pod and a Service. Install it and configure the controller
// to publish the load balancer IP address on each Ingress so that
// applications can depend on the IP address of the load balancer if needed.
const ctrl = new nginx.IngressController("myctrl", {
    controller: {
        
        publishService: {
            enabled: true,
        
    },
  },
});

const appBase = "oyelowo";
// Next, expose the app using an Ingress.
const appIngress = new k8s.networking.v1.Ingress(`${appBase}-ingress`, {
  metadata: {
    name: "hello-k8s-ingress",
    annotations: {
      "kubernetes.io/ingress.class": "nginx",
    },
  },
  spec: {
    rules: [
      {
        // Replace this with your own domain!
        // host: "myservicea.foo.org",
        host: "/app",
        http: {
          paths: [
            {
              pathType: "Prefix",
              path: "/",
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
      {
        // Replace this with your own domain!
        host: "myserviceb.foo.org",
        http: {
          paths: [
            {
              pathType: "Prefix",
              path: "/",
              backend: {
                service: {
                  name: graphqlPostgresSettings.resourceName,
                  port: { number: Number(graphqlPostgresEnvVars.APP_PORT) },
                },
              },
            },
          ],
        },
      },
    ],
  },
});

// export const appStatuses = apps;
export const controllerStatus = ctrl.status;

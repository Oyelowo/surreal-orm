import { getEnvironmentVariables } from "../../shared/validations";
// import { clusterSetupDirectory } from "../shared/manifestsDirectory";
import * as k8s from "@pulumi/kubernetes";

import { graphqlMongoSettings } from "../../services/graphql-mongo/settings";
import { reactWebSettings } from "../../services/react-web/settings";
// import { applicationsDirectory } from "../shared/manifestsDirectory";
import { IngressControllerValuesBitnami } from "../../shared/types/helm-charts/ingressControllerValuesBitnami";
import { namespaceNames } from "../../namespaces/util";
import { NginxConfiguration } from "../../shared/types/nginxConfigurations";
import { RecursivePartial } from "../../shared/types/own-types";
import { getIngressControllerDir, ingressControllerName } from "../../shared/manifestsDirectory";
import { DOMAIN_NAME_BASE } from "./constant"
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

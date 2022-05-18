import * as k8s from '@pulumi/kubernetes'
import { helmChartsInfo } from '../../shared/helmChartInfo'
import { IngressControllerValuesBitnami } from '../../shared/types/helm-charts/ingressControllerValuesBitnami'
import { RecursivePartial } from '../../shared/types/own-types'
import { nginxIngressProvider } from './settings'

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

const {
  repo,
  nginxIngress: { chart, version }
} = helmChartsInfo.bitnamiRepo
const ingressControllerValues: RecursivePartial<IngressControllerValuesBitnami> = {
  // containerPorts: {
  //   http: 8000,
  //   https: 443,
  // },
  fullnameOverride: chart,
  commonAnnotations: {
    'linkerd.io/inject': 'enabled'
  }
}
// nginx-ingress-controller
// K3s also comes with a traefik ingress controoler. Disable that if using this
export const ingressNginxController = new k8s.helm.v3.Chart(
  chart,
  {
    chart,
    fetchOpts: {
      repo
    },
    version,
    values: ingressControllerValues,
    // namespace: "nginx-ingress",
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false
  },
  { provider: nginxIngressProvider }
)

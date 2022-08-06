// import crds from './../../../generatedCrdsTs/index.js';
// import { namespaceNames } from "../../shared.js"
// import { CLUSTER_ISSUER_NAME } from "../cert-manager/clusterIssuer.js";
// import { DNS_NAME_LINODE_BASE } from "./constant.js"
// import { provider } from "./settings.js";

// export const SECRET_NAME_NGINX = "nginx-ingress-tls";

// export const certificateNginx = new crds.certmanager.v1.Certificate("certificate-nginx", {
//     metadata: {
//         name: "certificate-nginx",
//         namespace: namespaces.default
//     },
//     spec: {
//         dnsNames: [DNS_NAME_LINODE_BASE],
//         secretName: SECRET_NAME_NGINX,
//         issuerRef: {
//             name: CLUSTER_ISSUER_NAME,
//             kind: "ClusterIssuer"
//         }
//     }
// }, { provider: })

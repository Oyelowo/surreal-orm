/* An alternative for creating secrets for nginx-ingress. However, this is
automatically handled in the cert-manager cluster issuer.
This is left as a reference in case you want to create
arbitrary certificate for any other object using cert-manager
*/
// import crds from '../../../../generatedCrdsTs/index.js';
// import { CLUSTER_ISSUER_NAME } from '../cert-manager/clusterIssuer.js';
// import { namespaces } from '../namespaces/index.js';
// import { nginxIngressProvider } from './settings.js';

// const DNS_NAME_LINODE_BASE = 'oyelowo.dev';
// export const SECRET_NAME_NGINX = 'nginx-ingress-tls';

// export const certificateNginx = new crds.certmanager.v1.Certificate(
//     'certificate-nginx',
//     {
//         metadata: {
//             name: 'certificate-nginx',
//             namespace: namespaces.default,
//         },
//         spec: {
//             dnsNames: [DNS_NAME_LINODE_BASE],
//             secretName: SECRET_NAME_NGINX,
//             issuerRef: {
//                 name: CLUSTER_ISSUER_NAME,
//                 kind: 'ClusterIssuer',
//             },
//         },
//     },
//     { provider: nginxIngressProvider }
// );

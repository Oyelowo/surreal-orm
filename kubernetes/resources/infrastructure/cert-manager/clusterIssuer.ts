import * as cm from "../../../crd2pulumi/certManager/certmanager";
import { certManagerControllerProvider } from "./certManager";


export const CLUSTER_ISSUER_NAME = "letsencrypt-cluster-issuer"
export const clusterIssuer = new cm.v1.ClusterIssuer(
    "letsencrypt-cluster-issuer", {
    metadata: {
        // name: "letsencrypt-staging",
        name: CLUSTER_ISSUER_NAME,
        // namespace: "default"
    },
    spec: {
        acme: {
            // The ACME server URL
            // server: "https://acme-staging-v02.api.letsencrypt.org/directory",
            server: "https://acme-v02.api.letsencrypt.org/directory",
            // Email address used for ACME registration
            email: "oyelowooyedayo@gmail.com",
            // Name of a secret used to store the ACME account private key
            privateKeySecretRef:
            {
                // name: "letsencrypt-staging",
                name: `${CLUSTER_ISSUER_NAME}-key`,
            },
            // Enable the HTTP-01 challenge provider
            solvers: [{
                http01: {
                    ingress: {
                        class: "nginx"
                    }
                }
            }]

        }
    }
}, { provider: certManagerControllerProvider }
)

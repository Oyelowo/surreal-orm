import * as cm from "../../../crd2pulumi/certManager/certmanager";
import { certManagerControllerProvider } from "./certManager";

export const clusterIssuer = new cm.v1.ClusterIssuer(
    "letsencrypt-staging", {
    metadata: {
        name: "letsencrypt-staging",
        // namespace: "default"
    },
    spec: {
        acme: {
            // The ACME server URL
            server: "https://acme-staging-v02.api.letsencrypt.org/directory",
            // Email address used for ACME registration
            email: "oyelowooyedayo@gmail.com",
            // Name of a secret used to store the ACME account private key
            privateKeySecretRef:
            {
                name: "letsencrypt-staging",
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
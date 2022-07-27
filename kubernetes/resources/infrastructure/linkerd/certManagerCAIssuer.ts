import * as cm from './../../../generatedCrdsTs/certmanager';
import { namespaces } from '../namespaces/util';
import { certManagerProvider } from './../cert-manager/settings';

// ROOT TRUST ANCHOR CERTIFICATES AND CLUSTER ISSUE

export const CLUSTER_ISSUER_LINKERD_SELF_SIGNED_NAME = 'linkerd-self-signed-issuer';
export const LINKERD_TRUST_ANCHOR_CERTIFICATE_NAME = 'linkerd-trust-anchor';
export const LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME = 'linkerd-identity-trust-roots';
// First, create  a clusterIssuer for our linkerd Cert Authority. This resource will issue our root certificate
export const clusterIssuerLinkerdSelfSigned = new cm.v1.ClusterIssuer(
    CLUSTER_ISSUER_LINKERD_SELF_SIGNED_NAME,
    {
        metadata: {
            name: CLUSTER_ISSUER_LINKERD_SELF_SIGNED_NAME,
            // This should be in cert manager namespace because we want the certificate key to stay in
            //  that namespace rather than linkerd namespace
            namespace: namespaces.certManager,
        },
        spec: {
            selfSigned: {},

            // acme: {
            //     // The ACME server URL
            //     server: "https://acme-v02.api.letsencrypt.org/directory",
            //     // server: "https://acme-staging-v02.api.letsencrypt.org/directory",
            //     // server: "https://acme-v02.api.letsencrypt.org/directory",
            //     // Email address used for ACME registration
            //     email: "oyelowooyedayo@gmail.com",
            //     // Name of a secret used to store the ACME account private key
            //     privateKeySecretRef:
            //     {
            //         // name: "letsencrypt-staging",
            //         name: `${CLUSTER_ISSUER_NAME}-key`,
            //     },
            //     // Enable the HTTP-01 challenge provider
            //     solvers: [{
            //         http01: {
            //             ingress: {
            //                 class: ingressClassName
            //             }
            //         }
            //     }]

            // }
        },
        // We are using the certManager provider because we want it in that namespace anyway
    },
    { provider: certManagerProvider }
);

// Then, create the actual CA certificate to be used for validation paths.This
//  will be signed(issued) by our issuer created above,
export const certificateLinkerdTrustAnchor = new cm.v1.Certificate(
    LINKERD_TRUST_ANCHOR_CERTIFICATE_NAME,
    {
        metadata: {
            name: LINKERD_TRUST_ANCHOR_CERTIFICATE_NAME,
            namespace: namespaces.certManager,
        },
        spec: {
            isCA: true,
            commonName: 'root.linkerd.cluster.local',
            secretName: LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME,
            // renewBefore: "",
            privateKey: {
                algorithm: 'ECDSA',
                size: 256,
            },
            issuerRef: {
                name: CLUSTER_ISSUER_LINKERD_SELF_SIGNED_NAME,
                kind: 'ClusterIssuer',
                group: 'cert-manager.io',
            },
        },
    },
    { provider: certManagerProvider }
);

/*
Finally, create another ClusterIssuer to sign intermediate issuers. This
will use the root cert we just created, our issuer will be "signed" by the root CA.
*/
export const clusterIssuerLinkerdTrustAnchor = new cm.v1.ClusterIssuer(
    LINKERD_TRUST_ANCHOR_CERTIFICATE_NAME,
    {
        metadata: {
            name: LINKERD_TRUST_ANCHOR_CERTIFICATE_NAME,
            namespace: namespaces.certManager,
        },
        spec: {
            ca: {
                // This uses the certicate stored in the secret generated
                // by the certificate above, to sign other certificates
                secretName: LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME,
            },
        },
    },
    { provider: certManagerProvider }
);

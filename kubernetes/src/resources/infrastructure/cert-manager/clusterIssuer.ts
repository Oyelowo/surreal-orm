import crds from '../../../../generatedCrdsTs/index.js';
import { getEnvironmentVariables } from '../../shared/validations.js';
import { INGRESS_CLASSNAME_NGINX } from '../ingress/ingressRules.js';

import { certManagerProvider } from './settings.js';
const { ENVIRONMENT } = getEnvironmentVariables();
export const CLUSTER_ISSUER_NAME = 'letsencrypt-cluster-issuer';

const acme: crds.types.input.certmanager.v1.ClusterIssuerSpecAcmeArgs = {
    // The ACME server URL
    // Comment: Could also use self signed cert for local development
    server:
        ENVIRONMENT === 'production'
            ? 'https://acme-v02.api.letsencrypt.org/directory'
            : 'https://acme-staging-v02.api.letsencrypt.org/directory',
    // server: "https://acme-v02.api.letsencrypt.org/directory",
    // Email address used for ACME registration
    email: 'oyelowooyedayo@gmail.com',
    // Name of a secret used to store the ACME account private key
    privateKeySecretRef: {
        name: `${CLUSTER_ISSUER_NAME}-key`,
    },
    // Enable the HTTP-01 challenge provider
    solvers: [
        {
            http01: {
                ingress: {
                    class: INGRESS_CLASSNAME_NGINX,
                },
            },
        },
    ],
};

export const clusterIssuer = new crds.certmanager.v1.ClusterIssuer(
    'letsencrypt-cluster-issuer',
    {
        metadata: {
            name: CLUSTER_ISSUER_NAME,
        },
        spec: ENVIRONMENT === 'local' || ENVIRONMENT === 'test' ? { selfSigned: {} } : { acme },
    },
    { provider: certManagerProvider }
);

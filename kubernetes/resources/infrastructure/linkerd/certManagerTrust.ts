import { linkerdProvider } from "./linkerd";
import { namespaceNames } from "../../namespaces/namespaces";
import * as cmt from "../../../crd2pulumi/certManagerTrust/trust";
import { LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME } from "./certManagerCAIssuer";

export const linkerdCertManagertrust = new cmt.v1alpha1.Bundle(
    LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME,
    {
        metadata: {
            name: LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME,
            namespace: namespaceNames.default,
        },
        spec: {
            sources: [
                {
                    secret: {
                        name: LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME,
                        // This takes just the certificate of the trust roots and not the key and distributes it
                        // in the cluster
                        key: "ca.crt",
                    },
                },
            ],
            target: {
                configMap: {
                    key: "ca-bundle.crt",
                },
            },
        },
    },
    { provider: linkerdProvider }
);

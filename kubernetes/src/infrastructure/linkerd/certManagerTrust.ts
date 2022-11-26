import crds from "../../../generatedCrdsTs/index.js";

import { certManagerProvider } from "../cert-manager/settings.js";
import { LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME } from "./certManagerCaIssuer.js";

// Distribute the public key of the identity anchor trust trust from secrets to config maps
// across clusters/ i.e in every namespace
export const linkerdCertManagertrust = new crds.trust.v1alpha1.Bundle(
	LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME,
	{
		metadata: {
			name: LINKERD_IDENTITY_TRUST_ROOTS_SECRET_NAME,
			// namespace: namespaces.default,
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
	// Put in cert manager so that it can be redistributed earlier
	{ provider: certManagerProvider },
);

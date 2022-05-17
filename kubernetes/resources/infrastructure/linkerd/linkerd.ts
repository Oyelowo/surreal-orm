import { ResourceName } from "./../../shared/manifestsDirectory";
import { helmChartsInfo } from "./../../shared/helmChartInfo";
import { Linkerd2HelmValues } from "../../shared/types/helm-charts/linkerd2HelmValues";
import * as k8s from "@pulumi/kubernetes";

import { namespaceNames } from "../../namespaces/util";
import { DeepPartial } from "../../shared/types/own-types";
import { linkerdProvider } from "./settings";
/* 
 --set-file identityTrustAnchorsPEM=ca.crt \
  --set-file identity.issuer.tls.crtPEM=issuer.crt \
  --set-file identity.issuer.tls.keyPEM=issuer.key \
*/

/* 
for automanaged ca
  --set-file identityTrustAnchorsPEM=ca.crt \
  --set identity.issuer.scheme=kubernetes.io/tls \
  --set installNamespace=false \
  linkerd/linkerd2 \
*/
const Linkerd2Values: DeepPartial<Linkerd2HelmValues> = {
  // identityTrustAnchorsPEM: "ca.crt",
  // identityTrustDomain
  // installNamespace: false,
  podAnnotations: {
    "sealedsecrets.bitnami.com/managed": "true",
  },
  identity: {
    externalCA: true,
    issuer: {
      scheme: "kubernetes.io/tls",
      // tls: {
      //   crtPEM: "",
      //   keyPEM: "",
      // },
    },
  },
  // proxyInit:{
  //   runAsRoot: true
  // }
  // cniEnabled: true
};

// const resourceName: ResourceName = "linkerd";
const {
  repo,
  linkerd2: { chart, version },
} = helmChartsInfo.linkerdRepo;
export const linkerd = new k8s.helm.v3.Chart(
  chart,
  {
    chart,
    fetchOpts: {
      repo,
    },
    version,
    values: Linkerd2Values,
    namespace: namespaceNames.linkerd,
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: linkerdProvider }
);

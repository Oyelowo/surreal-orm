import { helmChartsInfo } from './../../shared/helmChartInfo';
import { Linkerd2HelmValues } from "../../shared/types/helm-charts/linkerd2HelmValues";
import * as k8s from "@pulumi/kubernetes";

import { getLinkerd2Dir, linkerd2Name } from "../../shared/manifestsDirectory";
import { namespaceNames } from "../../namespaces/namespaces";
import { DeepPartial, RecursivePartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";


export const linkerdDir = getLinkerd2Dir(
  getEnvironmentVariables().ENVIRONMENT
);

export const linkerdProvider = new k8s.Provider(linkerdDir, {
  renderYamlToDirectory: linkerdDir,
});

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
  identityTrustAnchorsPEM: "ca.crt",
  identity: {
    issuer: {
      scheme: "kubernetes.io/tls",
      // tls: {
      //   crtPEM: "",
      //   keyPEM: "",
      // },
    }
  },
  // cniEnabled: true
};

const { repo, linkerd2: { chart, version } } = helmChartsInfo.linkerdRepo;
export const linkerd = new k8s.helm.v3.Chart(
  linkerd2Name,
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
  // { provider }
);

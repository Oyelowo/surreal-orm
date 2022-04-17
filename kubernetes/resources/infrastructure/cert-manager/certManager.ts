import * as k8s from "@pulumi/kubernetes";
import { getCertManagerControllerDir } from "../../shared/manifestsDirectory";
import { namespaceNames } from "../../shared/namespaces";
import { DeepPartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";
import { CertManagerValuesJetspack } from "../../shared/types/helm-charts/certManagerValuesJetspack";
import { CertManagerValuesBitnami } from "../../shared/types/helm-charts/certManagerValuesBitnami";

const { ENVIRONMENT } = getEnvironmentVariables();
const certManagerControllerDir = getCertManagerControllerDir(ENVIRONMENT);

// type Metadata = {
//   name: string;
//   namespace: string;
// };
// const metadata: Metadata = {
//   name: "cert-manager",
//   namespace: namespaceNames.certManager,
// };

// const resourceName = metadata.name;

export const certManagerControllerProvider = new k8s.Provider(certManagerControllerDir, {
  renderYamlToDirectory: certManagerControllerDir,
});

// export const argoApplicationSecret = new k8s.

// CertManagerValuesBitnami
// const certManagerValuesB: DeepPartial<CertManagerValuesBitnami> = {
//   // fullnameOverride: "cert-manager",
//   fullnameOverride: "cert-manager",
//   installCRDs: true
// };
const certManagerValues: DeepPartial<CertManagerValuesJetspack> = {
  // fullnameOverride: "cert-manager",
  installCRDs: true
};

// `http://${name}.${namespace}:${port}`;
export const certManagerHelm = new k8s.helm.v3.Chart(
  "certManager",
  {
    chart: "cert-manager",
    fetchOpts: {
      repo: "https://charts.jetstack.io",
    },
    version: "1.8.0",
    values: certManagerValues,
    namespace: namespaceNames.certManager,
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: certManagerControllerProvider }
  // { provider }
);

import { CertManagerValuesBitnami } from './../../shared/types/helm-charts/certManagerValuesBitnami';
import { helmChartsMetadata } from './../../shared/helmChartInfo';
import * as k8s from "@pulumi/kubernetes";
import { getCertManagerControllerDir } from "../../shared/manifestsDirectory";
import { namespaceNames } from "../../shared/namespaces";
import { DeepPartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";
import { CertManagerValuesJetspack } from "../../shared/types/helm-charts/certManagerValuesJetspack";

const { ENVIRONMENT } = getEnvironmentVariables();
export const certManagerControllerDir = getCertManagerControllerDir(ENVIRONMENT);


export const certManagerControllerProvider = new k8s.Provider(certManagerControllerDir, {
  renderYamlToDirectory: certManagerControllerDir,
});


const certManagerValues: DeepPartial<CertManagerValuesBitnami> = {
  installCRDs: true
};

export const certManagerHelm = new k8s.helm.v3.Chart(
  "cert-manager",
  {
    chart: helmChartsMetadata.certManager.bitnami.version,
    fetchOpts: {
      // repo: "https://charts.jetstack.io",
      repo: helmChartsMetadata.certManager.bitnami.repoUrl,
    },
    // version: "1.8.0",
    version: helmChartsMetadata.certManager.bitnami.version,
    values: certManagerValues,
    namespace: namespaceNames.certManager,
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: certManagerControllerProvider }
);

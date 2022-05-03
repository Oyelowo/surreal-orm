import { getPathToResource, createProvider } from './../../shared/manifestsDirectory';
import { helmChartsInfo } from './../../shared/helmChartInfo';
import * as k8s from "@pulumi/kubernetes";
import { namespaceNames } from "../../namespaces/util";
import { DeepPartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";
import { CertManagerValuesJetspack } from "../../shared/types/helm-charts/certManagerValuesJetspack";

const { ENVIRONMENT } = getEnvironmentVariables();


export const certManagerControllerProvider = createProvider({
  resourceName: "cert-manager",
  resourceType: "infrastructure",
  environment: ENVIRONMENT
})


const certManagerValues: DeepPartial<CertManagerValuesJetspack> = {
  installCRDs: true,
};

export const certManagerHelm = new k8s.helm.v3.Chart(
  "cert-manager",
  {
    chart: helmChartsInfo.jetspackRepo.certManager.chart,
    fetchOpts: {
      // repo: "https://charts.jetstack.io",
      repo: helmChartsInfo.jetspackRepo.repo,
    },
    // version: "1.8.0",
    version: helmChartsInfo.jetspackRepo.certManager.version,
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

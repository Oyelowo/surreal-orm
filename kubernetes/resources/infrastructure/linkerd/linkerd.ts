import { helmChartsMetadata } from '../../shared/helmChartInfo';
import { Linkerd2HelmValues } from "../../shared/types/helm-charts/linkerd2HelmValues";
import * as k8s from "@pulumi/kubernetes";

import { getLinkerd2Dir, linkerd2Name } from "../../shared/manifestsDirectory";
import { namespaceNames } from "../../shared/namespaces";
import { DeepPartial, RecursivePartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";


export const linkerd2Dir = getLinkerd2Dir(
  getEnvironmentVariables().ENVIRONMENT
);

const Linkerd2Provider = new k8s.Provider(linkerd2Dir, {
  renderYamlToDirectory: linkerd2Dir,
});

const Linkerd2Values: DeepPartial<Linkerd2HelmValues> = {


};

export const linkerd2 = new k8s.helm.v3.Chart(
  linkerd2Name,
  {
    chart: helmChartsMetadata.linked2.linkerd.chart,
    fetchOpts: {
      repo: helmChartsMetadata.linked2.linkerd.repo,
    },
    version: helmChartsMetadata.linked2.linkerd.version,
    values: Linkerd2Values,
    namespace: namespaceNames.linkerd,
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: Linkerd2Provider }
  // { provider }
);

import { certManagerProvider } from './settings';
import { CertManagerTrustHelmValues } from './../../shared/types/helm-charts/certManagerTrustHelmValues';
import { helmChartsInfo } from '../../shared/helmChartInfo';
import * as k8s from '@pulumi/kubernetes';
import { namespaceNames } from '../../namespaces/util';
import { DeepPartial } from '../../shared/types/own-types';

const values: DeepPartial<CertManagerTrustHelmValues> = {};
const {
    repo,
    certManagerTrust: { chart, version },
} = helmChartsInfo.jetspackRepo;
export const certManagerTrustDeploymentName = chart;
export const certManagerTrustHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values,
        namespace: namespaceNames.certManager,
        // namespace: devNamespaceName,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: certManagerProvider }
);

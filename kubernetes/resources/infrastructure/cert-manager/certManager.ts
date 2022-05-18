import { helmChartsInfo } from './../../shared/helmChartInfo';
import * as k8s from '@pulumi/kubernetes';
import { namespaceNames } from '../../namespaces/util';
import { DeepPartial } from '../../shared/types/own-types';
import { CertManagerValuesJetspack } from '../../shared/types/helm-charts/certManagerValuesJetspack';
import { certManagerProvider } from './settings';

const certManagerValues: DeepPartial<CertManagerValuesJetspack> = {
    installCRDs: true,
};

const {
    repo,
    certManager: { chart, version },
} = helmChartsInfo.jetspackRepo;
export const certManagerHelm = new k8s.helm.v3.Chart(
    'cert-manager',
    {
        chart,
        fetchOpts: {
            // repo: "https://charts.jetstack.io",
            repo,
        },
        // version: "1.8.0",
        version,
        values: certManagerValues,
        namespace: namespaceNames.certManager,
        // namespace: devNamespaceName,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: certManagerProvider }
);

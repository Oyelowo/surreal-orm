import { ICertmanagerbitnami } from './../../types/helm-charts/certManagerBitnami';
import { helmChartsInfo } from './../../shared/helmChartInfo';
import * as k8s from '@pulumi/kubernetes';
import { namespaceNames } from '../../namespaces/util';
import { DeepPartial } from '../../types/own-types';
import { certManagerProvider } from './settings';

const certManagerValues: DeepPartial<ICertmanagerbitnami> = {
    installCRDs: true,
};

const {
    repo,
    charts: {
        certManager: { chart, version },
    },
} = helmChartsInfo.bitnami;
export const certManagerHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: certManagerValues,
        namespace: namespaceNames.certManager,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: certManagerProvider }
);

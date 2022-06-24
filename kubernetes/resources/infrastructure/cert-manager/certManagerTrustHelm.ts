import { ICertmanagertrustjetstack } from './../../types/helm-charts/certManagerTrustJetstack';
import { certManagerProvider } from './settings';
import { helmChartsInfo } from '../../shared/helmChartInfo';
import * as k8s from '@pulumi/kubernetes';
import { namespaceNames } from '../../namespaces/util';
import { DeepPartial } from '../../types/own-types';

const values: DeepPartial<ICertmanagertrustjetstack> = {};

const {
    repo,
    charts: {
        certManagerTrust: { chart, version },
    },
} = helmChartsInfo.jetstack;

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
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: certManagerProvider }
);

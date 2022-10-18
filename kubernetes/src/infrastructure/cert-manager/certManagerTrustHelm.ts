import { ICertManagerTrustJetstack } from '../../../generatedHelmChartsTsTypes/certManagerTrustJetstack.js';
import { certManagerProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../../types/ownTypes.js';
import { DeepPartial } from '../../types/ownTypes.js';

const values: DeepPartial<ICertManagerTrustJetstack> = {};

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
        namespace: namespaces.certManager,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: certManagerProvider }
);

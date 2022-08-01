import { ICertmanagertrustjetstack } from '../../../generatedHelmChartsTsTypes/certManagerTrustJetstack.js';
import { certManagerProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { DeepPartial } from '../../types/own-types.js';

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
        namespace: namespaces.certManager,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: certManagerProvider }
);

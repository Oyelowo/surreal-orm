import { ICertmanagerbitnami } from '../../../../generatedHelmChartsTsTypes/certManagerBitnami.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { certManagerProvider } from './settings.js';

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
        namespace: namespaces.certManager,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: certManagerProvider }
);

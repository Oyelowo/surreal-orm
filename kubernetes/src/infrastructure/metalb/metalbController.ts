import { IMetallbbitnami } from '../../../generatedHelmChartsTsTypes/metallbBitnami.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { metalbProvider } from './settings.js';

const metalbOperatValues: DeepPartial<IMetallbbitnami> = {};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        metalb: { chart, version },
    },
} = helmChartsInfo.bitnami;

export const metalbController = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: metalbOperatValues,
        namespace: namespaces.metalb,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: metalbProvider }
);

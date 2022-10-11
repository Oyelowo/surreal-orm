import { IRookCephRookCeph } from '../../../generatedHelmChartsTsTypes/rookCephRookceph.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { rookCephProvider } from './settings.js';

const rookCephOperatValues: DeepPartial<IRookCephRookCeph> = {};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        rookCephOperator: { chart, version },
    },
} = helmChartsInfo.rookCeph;

export const rookCephOperator = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: rookCephOperatValues,
        namespace: namespaces.rookCeph,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: rookCephProvider }
);

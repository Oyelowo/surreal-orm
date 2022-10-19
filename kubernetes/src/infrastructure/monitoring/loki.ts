import { ILokiDistributedGrafana } from '../../../generatedHelmChartsTsTypes/lokiDistributedGrafana.js';
import * as k8s from '@pulumi/kubernetes';
import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import { monitoringProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';

const lokiValues: DeepPartial<ILokiDistributedGrafana> = {};

const {
    repo,
    charts: {
        loki: { chart, version },
    },
} = helmChartsInfo.grafana;

export const lokiHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: lokiValues,
        namespace: namespaces.monitoring,
        skipAwait: false,
    },
    { provider: monitoringProvider }
);

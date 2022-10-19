import { ITempoDistributedGrafana } from '../../../generatedHelmChartsTsTypes/tempoDistributedGrafana.js';
import * as k8s from '@pulumi/kubernetes';
import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import { monitoringProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';

const tempoValues: DeepPartial<ITempoDistributedGrafana> = {};

const {
    repo,
    charts: {
        tempo: { chart, version },
    },
} = helmChartsInfo.grafana;

export const tempoHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: tempoValues,
        namespace: namespaces.monitoring,
        skipAwait: false,
    },
    { provider: monitoringProvider }
);

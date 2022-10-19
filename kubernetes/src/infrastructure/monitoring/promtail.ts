import { IPromtailGrafana } from '../../../generatedHelmChartsTsTypes/promtailGrafana.js';
import * as k8s from '@pulumi/kubernetes';
import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import { monitoringProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';

const promtailValues: DeepPartial<IPromtailGrafana> = {};

const {
    repo,
    charts: {
        promtail: { chart, version },
    },
} = helmChartsInfo.grafana;

export const promtailHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: promtailValues,
        namespace: namespaces.monitoring,
        skipAwait: false,
    },
    { provider: monitoringProvider }
);

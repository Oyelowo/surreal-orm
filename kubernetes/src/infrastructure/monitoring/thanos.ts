import { IThanosBitnami } from '../../../generatedHelmChartsTsTypes/thanosBitnami.js';
import * as k8s from '@pulumi/kubernetes';
import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import { monitoringProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';

const thanosValues: DeepPartial<IThanosBitnami> = {};

const {
    repo,
    charts: {
        thanos: { chart, version },
    },
} = helmChartsInfo.bitnami;

export const thanosHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: thanosValues,
        namespace: namespaces.monitoring,
        skipAwait: false,
    },
    { provider: monitoringProvider }
);

import { IHarborHarbor } from '../../../generatedHelmChartsTsTypes/harborHarbor.js';
import * as k8s from '@pulumi/kubernetes';
import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import { harborProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';

const harborValues: DeepPartial<IHarborHarbor> = {};

const {
    repo,
    charts: {
        harbor: { chart, version },
    },
} = helmChartsInfo.harbor;

export const harborHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: harborValues,
        namespace: namespaces.harbor,
        skipAwait: false,
    },
    { provider: harborProvider }
);

import { ICiliumCilium } from '../../../generatedHelmChartsTsTypes/ciliumCilium.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../../types/ownTypes.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { ciliumProvider } from './settings.js';

const ciliumValues: DeepPartial<ICiliumCilium> = {};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        cilium: { chart, version },
    },
} = helmChartsInfo.cilium;

export const ciliumOperator = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: ciliumValues,
        namespace: namespaces.kubeSystem,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: ciliumProvider }
);

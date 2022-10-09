import { ITidboperatorpingcap } from '../../../generatedHelmChartsTsTypes/tidbOperatorPingcap.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { fluvioSysProvider } from './settings.js';

const tikvOperatValues: DeepPartial<ITidboperatorpingcap> = {
    // advancedStatefulset : {}
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        fluvioSys: { chart, version },
    },
} = helmChartsInfo.oyelowo;

export const tikvOperator = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: tikvOperatValues,
        namespace: namespaces.fluvioSys,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: fluvioSysProvider }
);

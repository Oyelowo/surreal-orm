import { ITidboperatorpingcap } from '../../../generatedHelmChartsTsTypes/tidbOperatorPingcap.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { tikvProvider, tikvResourceName } from './settings.js';

const tikvOperatValues: DeepPartial<ITidboperatorpingcap> = {
    // advancedStatefulset : {}
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        tikvCluster: { chart, version },
    },
} = helmChartsInfo.pingcap;

export const tikvCluster = new k8s.helm.v3.Chart(
    tikvResourceName,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: tikvOperatValues,
        namespace: namespaces.kubeSystem,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: tikvProvider }
);

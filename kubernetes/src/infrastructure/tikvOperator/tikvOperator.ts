import { ITidbOperatorPingcap } from '../../../generatedHelmChartsTsTypes/tidbOperatorPingcap.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../../types/ownTypes.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { tikvOperatorProvider } from './settings.js';

const tikvOperatValues: DeepPartial<ITidbOperatorPingcap> = {
    // advancedStatefulset : {}
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        tikvOperator: { chart, version, externalCrds },
    },
} = helmChartsInfo.pingcap;

// CRDs
// Tikv/Tidb operator helm chart does not include the crds. That's why we're handling it separately
export const tikvCrds = new k8s.yaml.ConfigGroup(
    'tikv-operator-crd',
    {
        files: externalCrds,
    },
    { provider: tikvOperatorProvider }
);

export const tikvOperator = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: tikvOperatValues,
        namespace: namespaces.tikvAdmin,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: tikvOperatorProvider }
);

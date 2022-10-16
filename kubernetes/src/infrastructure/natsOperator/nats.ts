import { INatsNats } from '../../../generatedHelmChartsTsTypes/natsNats.js';
import * as k8s from '@pulumi/kubernetes';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import { natsOperatorProvider } from './settings.js';

const natsValues: DeepPartial<INatsNats> = {
    nats: {
        // client: {},
        jetstream: {
            enabled: true,
            fileStorage: {
                enabled: true,
                // existingClaim: 

            },
        },
        // externalAccess: false
    },
    exporter: {
        enabled: true
    },
    mqtt: {

    },
    websocket: {
        enabled: true
    },
    cluster: {
        enabled: true
    }
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        nats: { chart, version, externalCrds },
    },
} = helmChartsInfo.nats;


export const nats = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: natsValues,
        namespace: namespaces.natsOperator,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: natsOperatorProvider }
);

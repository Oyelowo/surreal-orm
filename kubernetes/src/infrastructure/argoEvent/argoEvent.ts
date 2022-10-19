import { IArgoEventsArgo } from '../../../generatedHelmChartsTsTypes/argoEventsArgo.js';
import * as k8s from '@pulumi/kubernetes';
import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import { argoEventProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';

const argoEventValues: DeepPartial<IArgoEventsArgo> = {
    crds: {
        install: true,
    },
};

const {
    repo,
    charts: {
        argoEvent: { chart, version },
    },
} = helmChartsInfo.argo;

export const argoEventHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: argoEventValues,
        namespace: namespaces.argoEvent,
        skipAwait: false,
    },
    { provider: argoEventProvider }
);

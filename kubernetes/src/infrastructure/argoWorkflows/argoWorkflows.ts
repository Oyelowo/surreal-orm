import { IArgoWorkflowsArgo } from '../../../generatedHelmChartsTsTypes/argoWorkflowsArgo.js';
import * as k8s from '@pulumi/kubernetes';
import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import { argoWorkflowsProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';

const argoWorkflowsValues: DeepPartial<IArgoWorkflowsArgo> = {
    crds: {
        install: true,
    },
};

const {
    repo,
    charts: {
        argoWorkflows: { chart, version },
    },
} = helmChartsInfo.argo;

export const argoWorkflowsHelm = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: argoWorkflowsValues,
        namespace: namespaces.argoWorkflows,
        skipAwait: false,
    },
    { provider: argoWorkflowsProvider }
);

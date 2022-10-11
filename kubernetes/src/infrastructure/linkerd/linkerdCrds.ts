import { ILinkerdCrdsLinkerd } from '../../../generatedHelmChartsTsTypes/linkerdCrdsLinkerd.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { linkerdProvider } from './settings.js';

const LinkerdCrdsValues: DeepPartial<ILinkerdCrdsLinkerd> = {};

const {
    repo,
    charts: {
        linkerdCrds: { chart, version },
    },
} = helmChartsInfo.linkerd;

export const linkerdCrds = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: LinkerdCrdsValues,
        namespace: namespaces.linkerd,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: linkerdProvider }
);

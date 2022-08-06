import { ILinkerd2linkerd } from '../../../generatedHelmChartsTsTypes/linkerd2Linkerd.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { DeepPartial } from '../../types/own-types.js';
import { helmChartsInfo } from './../../shared/helmChartInfo.js';
import { linkerdProvider } from './settings.js';

const Linkerd2Values: DeepPartial<ILinkerd2linkerd> = {
    podAnnotations: {
        // 'sealedsecrets.bitnami.com/managed': 'true',
    },
    identity: {
        externalCA: true,
        issuer: {
            scheme: 'kubernetes.io/tls',
        },
    },
};

const {
    repo,
    charts: {
        linkerd2: { chart, version },
    },
} = helmChartsInfo.linkerd;

export const linkerd = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: Linkerd2Values,
        namespace: namespaces.linkerd,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: linkerdProvider }
);

import { ingressControllerPorts } from './hosts';
import { INginxingresscontrollerbitnami } from './../../types/helm-charts/nginxIngressControllerBitnami';
import * as k8s from '@pulumi/kubernetes';
import { helmChartsInfo } from '../../shared/helmChartInfo';
import { RecursivePartial } from '../../types/own-types';
import { nginxIngressProvider } from './settings';

const {
    repo,
    charts: {
        nginxIngress: { chart, version },
    },
} = helmChartsInfo.bitnami;

const ingressControllerValues: RecursivePartial<INginxingresscontrollerbitnami> = {
    service: {
        ports: ingressControllerPorts,
    },
    fullnameOverride: chart,
    commonAnnotations: {
        'linkerd.io/inject': 'enabled',
    },
};

export const ingressNginxController = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: ingressControllerValues,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: nginxIngressProvider }
);

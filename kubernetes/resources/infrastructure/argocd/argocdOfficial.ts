import { IArgocdargo } from './../../types/helm-charts/argoCdArgo';
import { annotations, INGRESS_CLASSNAME_NGINX } from '../ingress/ingressRules';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util';

import { DeepPartial } from '../../types/own-types';
import bcrypt from 'bcrypt';
import { DOMAIN_NAME_SUB_ARGOCD } from '../ingress/constant';
import { argocdProvider } from './settings';
import { helmChartsInfo } from '../../shared/helmChartInfo';

const saltRounds = 10;
const myPlaintextPassword = 'oyelowo';
const hash = bcrypt.hashSync(myPlaintextPassword, saltRounds);
const argocdValues: DeepPartial<IArgocdargo> = {
    fullnameOverride: 'argocd',
    server: {
        ingress: {
            enabled: true,
            ingressClassName: INGRESS_CLASSNAME_NGINX,
            annotations: {
                ...annotations,
            },
            https: true,
            tls: [
                {
                    hosts: [DOMAIN_NAME_SUB_ARGOCD],
                    secretName: `${DOMAIN_NAME_SUB_ARGOCD}-tls` as any,
                },
            ],
            hosts: [DOMAIN_NAME_SUB_ARGOCD] as any[],
        },
        // Ingress-controller already handles TLS. Argocd does too which causes collision. Disable argo from doing that
        // https://stackoverflow.com/questions/49856754/nginx-ingress-too-many-redirects-when-force-ssl-is-enabled
        extraArgs: ['--insecure'] as any[],
    },
    configs: {
        secret: {
            // createSecret: false,
            argocdServerAdminPassword: hash,
            annotations: {
                'sealedsecrets.bitnami.com/managed': 'true',
            },
        },
    },
    dex: {
        enabled: false,
    },
    redis: {
        enabled: true,
    },
    notifications: {
        enabled: false,
        secret: {
            create: true,
            items: {
                name: 'ererer',
            },
        },
    },
};

const {
    repo,
    charts: {
        argoCD: { chart, version },
    },
} = helmChartsInfo.argo;

export const argocdHelm = new k8s.helm.v3.Chart(
    'argocd',
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: argocdValues,
        namespace: namespaces.argocd,
        // namespace: devNamespaceName,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: argocdProvider }
);

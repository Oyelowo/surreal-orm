// import { IArgocdargo } from '../../../types/helm-charts/argoCdArgo.js';
import { IArgoCdArgo } from '../../../generatedHelmChartsTsTypes/argoCdArgo.js';
import { annotations } from '../ingress/ingressRules.js';
import * as k8s from '@pulumi/kubernetes';

import { DeepPartial, namespaces } from '../../types/ownTypes.js';
import bcrypt from 'bcrypt';
import { argocdProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { getIngressUrlHost } from '../ingress/hosts.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';
import { PlainSecretsManager } from '../../../scripts/utils/plainSecretsManager.js';
import { INGRESS_CLASSNAME_NGINX } from '../../types/nginxConfigurations.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();
const secrets = new PlainSecretsManager('infrastructure', 'argocd', ENVIRONMENT).getSecrets();
const argocdHost = getIngressUrlHost({ environment: ENVIRONMENT, subDomain: 'argocd' });
const saltRounds = 10;

const myPlaintextPassword = secrets.ADMIN_PASSWORD;
const hash = bcrypt.hashSync(myPlaintextPassword, saltRounds);

const argocdValues: DeepPartial<IArgoCdArgo> = {
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
                    hosts: [argocdHost],
                    secretName: `${argocdHost}-tls`,
                },
            ],
            hosts: [argocdHost],
        },
        // Ingress-controller already handles TLS. Argocd does too which causes collision. Disable argo from doing that
        // https://stackoverflow.com/questions/49856754/nginx-ingress-too-many-redirects-when-force-ssl-is-enabled
        extraArgs: ['--insecure'],
    },
    configs: {
        secret: {
            // createSecret: false,
            argocdServerAdminPassword: hash,
            annotations: {
                // 'sealedsecrets.bitnami.com/managed': 'true',
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

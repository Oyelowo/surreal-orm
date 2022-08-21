import { IArgocdbitnami } from '../../../generatedHelmChartsTsTypes/argoCdBitnami.js';
import { annotations } from '../ingress/ingressRules.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { DeepPartial, STORAGE_CLASS } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';
import { argocdProvider } from './settings.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { getIngressUrlHost } from '../ingress/hosts.js';
import { PlainKubeBuildSecretsManager } from '../../../scripts/utils/plainKubeBuildSecretsManager.js';
import { INGRESS_CLASSNAME_NGINX } from '../../types/nginxConfigurations.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();
const secrets = new PlainKubeBuildSecretsManager('infrastructure', 'argocd', ENVIRONMENT).getSecrets();
const argocdValuesOld: DeepPartial<IArgocdbitnami> = {
    config: {
        secret: {
            create: true,
            // TODO: Change
            argocdServerAdminPassword: secrets.ADMIN_PASSWORD,
            annotations: {
                // 'sealedsecrets.bitnami.com/managed': 'true',
            },
        },
    },
    global: {
        storageClass: ENVIRONMENT === 'local' ? '' : STORAGE_CLASS,
    },
    server: {
        ingress: {
            enabled: true,
            hostname: getIngressUrlHost({ environment: ENVIRONMENT, subDomain: 'argocd' }),
            annotations,
            pathType: 'Prefix' as 'Exact' | 'ImplementationSpecific' | 'Prefix',
            ingressClassName: INGRESS_CLASSNAME_NGINX,
            tls: true,
        },
        // Ingress-controller already handles TLS. Argocd does too which causes collision. Disable argo from doing that
        // https://stackoverflow.com/questions/49856754/nginx-ingress-too-many-redirects-when-force-ssl-is-enabled
        extraArgs: ['--insecure'],
    },
    dex: {
        enabled: false,
    },
};

const {
    repo,
    charts: {
        argocd: { chart, version },
    },
} = helmChartsInfo.bitnami;

export const argocdHelm = new k8s.helm.v3.Chart(
    'argocd',
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: argocdValuesOld,
        namespace: namespaces.argocd,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: argocdProvider }
);

import { ILinkerdvizlinkerd } from '../../../generatedHelmChartsTsTypes/linkerdVizLinkerd.js';
import * as k8s from '@pulumi/kubernetes';
import * as kx from '@pulumi/kubernetesx';
import * as bcrypt from 'bcrypt';
import { namespaces } from '../namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { toBase64 } from '../../shared/converters.js';
import { NginxConfiguration } from '../../types/nginxConfigurations.js';
import { DeepPartial, ResourceName } from '../../types/own-types.js';
import { CLUSTER_ISSUER_NAME } from '../cert-manager/clusterIssuer.js';
import { DOMAIN_NAME_SUB_LINKERD_VIZ } from '../ingress/constant.js';
import { INGRESS_CLASSNAME_NGINX } from '../ingress/ingressRules.js';
import { linkerdVizSecretsFromLocalConfigs, linkerdVizProvider } from './settings.js';

const values: DeepPartial<ILinkerdvizlinkerd> = {};
const resourceName: ResourceName = 'linkerd-viz';
const {
    repo,
    charts: {
        linkerdViz: { chart, version },
    },
} = helmChartsInfo.linkerd;
export const linkerdVizHelmChart = new k8s.helm.v3.Chart(
    resourceName,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values,
        namespace: namespaces.linkerdViz,
        // namespace: devNamespaceName,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: linkerdVizProvider }
);

const linkerdVizIngressName = 'linkerd-viz-ingress';
const linkerdVizSecretName = `${linkerdVizIngressName}-auth`;
const nginxAnnotions: Partial<NginxConfiguration> = {
    // "nginx.ingress.kubernetes.io/upstream-vhost":
    //     "$service_name.$namespace.svc.cluster.local:8084",
    // "nginx.ingress.kubernetes.io/configuration-snippet": `|
    // proxy_set_header Origin;
    //   proxy_hide_header l5d-remote-ip;
    //   proxy_hide_header l5d-server-id;`,
    // "nginx.ingress.kubernetes.io/auth-type": "basic",
    // "nginx.ingress.kubernetes.io/auth-secret": linkerdVizSecretName,
    // "nginx.ingress.kubernetes.io/auth-realm": "Authentication Required",
};

const SECRET_NAME_NGINX = 'linkerd-nginx-ingress-tls';
export const linkerVizIngress = new k8s.networking.v1.Ingress(
    'linkerd-viz-ingress',
    {
        metadata: {
            name: linkerdVizIngressName,
            namespace: namespaces.linkerdViz,
            annotations: {
                ...(nginxAnnotions as Record<string, string>),
                'cert-manager.io/cluster-issuer': CLUSTER_ISSUER_NAME,
            },
        },
        spec: {
            ingressClassName: INGRESS_CLASSNAME_NGINX,
            tls: [
                {
                    hosts: [DOMAIN_NAME_SUB_LINKERD_VIZ],
                    secretName: SECRET_NAME_NGINX,
                },
            ],
            rules: [
                {
                    host: DOMAIN_NAME_SUB_LINKERD_VIZ,
                    http: {
                        paths: [
                            {
                                path: '/',
                                pathType: 'Prefix',
                                backend: {
                                    service: {
                                        name: 'web',
                                        port: {
                                            number: 8084,
                                        },
                                    },
                                },
                            },
                        ],
                    },
                },
            ],
        },
    },
    { provider: linkerdVizProvider }
);

const saltRounds = 10;

const hash = bcrypt.hashSync(linkerdVizSecretsFromLocalConfigs.PASSWORD, saltRounds);
export const linkerdVizSecret = new kx.Secret(
    linkerdVizSecretName,
    {
        metadata: {
            name: linkerdVizSecretName,
            namespace: namespaces.linkerdViz,
        },
        data: {
            // format: username:encryptedpassword
            // format: admin:$apr1$n7Cu6gHl$E47ogf7CO8NRYjEjBOkWM.
            auth: toBase64(`admin:${hash}`),
        },
    },
    { provider: linkerdVizProvider }
);

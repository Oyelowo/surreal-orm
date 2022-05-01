import { getLinkerdVizDir, linkerdVizName } from '../../shared/manifestsDirectory';
import * as bcrypt from "bcrypt";
import { INGRESS_CLASSNAME_NGINX } from "../ingress-controller/ingressRules";
import { NginxConfiguration } from "../../shared/types/nginxConfigurations";
import { LinkerdVizHelmValues } from "../../shared/types/helm-charts/linkerdVizHelmValues";
import { helmChartsInfo } from "../../shared/helmChartInfo";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { CLUSTER_ISSUER_NAME } from "../cert-manager/clusterIssuer";
import { DOMAIN_NAME_SUB_LINKERD_VIZ } from "../ingress-controller/constant";
import { getEnvironmentVariables } from "../../shared/validations";
import { namespaceNames } from "../../namespaces/namespaces";
import { DeepPartial } from "../../shared/types/own-types";



export const linkerdVizDir = getLinkerdVizDir(
    getEnvironmentVariables().ENVIRONMENT
);

export const linkerdVizProvider = new k8s.Provider(linkerdVizDir, {
    renderYamlToDirectory: linkerdVizDir,
});


const values: DeepPartial<LinkerdVizHelmValues> = {};

const {
    repo,
    linkerdViz: { chart, version },
} = helmChartsInfo.linkerdRepo;
export const linkerdVizHelmChart = new k8s.helm.v3.Chart(
    linkerdVizName,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values,
        namespace: namespaceNames.linkerdViz,
        // namespace: devNamespaceName,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: linkerdVizProvider }
);

const linkerdVizIngressName = "linkerd-viz-ingress";
const linkerdVizSecretName = `${linkerdVizIngressName}-auth`;
const nginxAnnotions: Partial<NginxConfiguration> = {
    "nginx.ingress.kubernetes.io/upstream-vhost":
        "$service_name.$namespace.svc.cluster.local:8084",
    "nginx.ingress.kubernetes.io/configuration-snippet": `|
    proxy_set_header Origin;
      proxy_hide_header l5d-remote-ip;
      proxy_hide_header l5d-server-id;`,
    "nginx.ingress.kubernetes.io/auth-type": "basic",
    "nginx.ingress.kubernetes.io/auth-secret": linkerdVizSecretName,
    "nginx.ingress.kubernetes.io/auth-realm": "Authentication Required",
};

const SECRET_NAME_NGINX = "linkerd-nginx-ingress-tls";
export const linkerVizIngress = new k8s.networking.v1.Ingress(
    "linkerd-viz-ingress",
    {
        metadata: {
            name: linkerdVizIngressName,
            namespace: namespaceNames.linkerdViz,
            annotations: {
                ...(nginxAnnotions as Record<string, string>),
                "cert-manager.io/cluster-issuer": CLUSTER_ISSUER_NAME,
            },
        },
        spec: {
            ingressClassName: INGRESS_CLASSNAME_NGINX,
            tls: [
                {
                    hosts: [DOMAIN_NAME_SUB_LINKERD_VIZ],
                    secretName: SECRET_NAME_NGINX

                }
            ],
            rules: [
                {
                    host: DOMAIN_NAME_SUB_LINKERD_VIZ,
                    http: {
                        paths: [
                            {
                                path:"/",
                                pathType: "Prefix",
                                backend: {
                                    service: {
                                        name: "web",
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
// TODO: Change alongside argocd
const myPlaintextPassword = "oyelowo";
const hash = bcrypt.hashSync(myPlaintextPassword, saltRounds);
export const linkerdVizSecret = new kx.Secret(
    linkerdVizSecretName,
    {
        metadata: {
            name: linkerdVizSecretName,
            namespace: namespaceNames.linkerdViz,
        },
        stringData: {
            // format: username:encryptedpassword
            // format: admin:$apr1$n7Cu6gHl$E47ogf7CO8NRYjEjBOkWM.
            auth: `admin:${hash}`,
        },
    },
    { provider: linkerdVizProvider }
);

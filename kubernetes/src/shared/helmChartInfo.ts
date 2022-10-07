type Repo = 'pingcap' | 'bitnami' | 'jetstack' | 'linkerd' | 'sealedSecrets' | 'argo';

type Chart = {
    chart: string;
    version: string;
    externalCrds?: string[]
};

type AllChartsInfo = Record<
    Repo,
    {
        repo: string;
        charts: Record<string, Chart>;
    }
>;

export const helmChartsInfo = {
    pingcap: {
        repo: 'https://charts.pingcap.org/',
        charts: {
            tikvOperator: {
                chart: 'tidb-operator',
                version: 'v1.3.8',
                externalCrds: [`https://raw.githubusercontent.com/pingcap/tidb-operator/v1.3.8/manifests/crd.yaml`]
            },
            tikvCluster: {
                chart: 'tidb-cluster',
                version: 'v1.3.8',
            },
        },
    },
    bitnami: {
        repo: 'https://charts.bitnami.com/bitnami',
        charts: {
            redis: {
                chart: 'redis',
                version: '16.8.9',
            },
            mongodb: {
                chart: 'mongodb',
                version: '11.1.10',
            },
            certManager: {
                chart: 'cert-manager',
                version: '0.6.1',
            },
            nginxIngress: {
                chart: 'nginx-ingress-controller',
                version: '9.2.11',
            },
            argocd: {
                chart: 'argo-cd',
                version: '4.0.6',
            },
            postgresql: {
                chart: 'postgresql',
                version: '11.6.7',
            },
            postgresqlHA: {
                chart: 'postgresql-ha',
                version: '9.1.6',
            },
        },
    },
    sealedSecrets: {
        repo: 'https://bitnami-labs.github.io/sealed-secrets',
        charts: {
            sealedSecrets: {
                chart: 'sealed-secrets',
                version: '2.1.7',
            },
        },
    },
    jetstack: {
        repo: 'https://charts.jetstack.io',
        charts: {
            certManager: {
                chart: 'cert-manager',
                version: 'v1.8.2',
            },
            certManagerTrust: {
                chart: 'cert-manager-trust',
                version: 'v0.1.1',
            },
        },
    },
    linkerd: {
        repo: 'https://helm.linkerd.io/stable',
        charts: {
            linkerd2: {
                chart: 'linkerd2',
                version: '2.11.2',
            },
            linkerdViz: {
                chart: 'linkerd-viz',
                version: '2.11.2',
            },
        },
    },
    argo: {
        repo: 'https://argoproj.github.io/argo-helm',
        charts: {
            argoCD: {
                chart: 'argo-cd',
                version: '4.5.3',
            },
        },
    },
} satisfies AllChartsInfo;

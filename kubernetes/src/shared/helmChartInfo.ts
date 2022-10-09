type Repo = 'oyelowo' | 'pingcap' | 'bitnami' | 'jetstack' | 'linkerd' | 'sealedSecrets' | 'argo';

type ChartInfo = {
    chart: string;
    version: string;
    externalCrds: string[]
    skipCrdRender: boolean
};

type ChartsInfo = Record<
    Repo,
    {
        repo: string;
        charts: Record<string, ChartInfo>;
    }
>;

export const helmChartsInfo = {
    oyelowo: {
        repo: 'https://oyelowo.github.io',
        charts: {
            seaweedfs: {
                chart: 'seaweedfs',
                version: '3.29',
                externalCrds: [] as string[],
                skipCrdRender: false
            },
            fluvioSys: {
                chart: 'fluvio-sys',
                version: '0.9.10',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
        },
    },
    pingcap: {
        repo: 'https://charts.pingcap.org/',
        charts: {
            tikvOperator: {
                chart: 'tidb-operator',
                version: 'v1.3.8',
                externalCrds: ["https://raw.githubusercontent.com/pingcap/tidb-operator/v1.3.8/manifests/crd.yaml"],
                skipCrdRender: false,
            },
            tikvCluster: {
                chart: 'tidb-cluster',
                version: 'v1.3.8',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
        },
    },
    bitnami: {
        repo: 'https://charts.bitnami.com/bitnami',
        charts: {
            redis: {
                chart: 'redis',
                version: '17.3.2',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            mongodb: {
                chart: 'mongodb',
                version: '11.1.10',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            certManager: {
                chart: 'cert-manager',
                version: '0.8.4',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            nginxIngress: {
                chart: 'nginx-ingress-controller',
                version: '9.3.18',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            argocd: {
                chart: 'argo-cd',
                version: '4.2.3',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            postgresql: {
                chart: 'postgresql',
                version: '11.9.8',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            postgresqlHA: {
                chart: 'postgresql-ha',
                version: '9.4.6',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
        },
    },
    sealedSecrets: {
        repo: 'https://bitnami-labs.github.io/sealed-secrets',
        charts: {
            sealedSecrets: {
                chart: 'sealed-secrets',
                version: '2.6.9',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
        },
    },
    jetstack: {
        repo: 'https://charts.jetstack.io',
        charts: {
            certManager: {
                chart: 'cert-manager',
                version: 'v1.9.1',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            certManagerTrust: {
                chart: 'cert-manager-trust',
                version: 'v0.2.0',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
        },
    },
    linkerd: {
        repo: 'https://helm.linkerd.io/stable',
        charts: {
            linkerdCrds: {
                chart: 'linkerd-crds',
                version: '1.4.0',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            linkerdControlPlane: {
                chart: 'linkerd-control-plane',
                version: '1.9.3',
                externalCrds: [] as string[],
                skipCrdRender: true
            },
            linkerdViz: {
                chart: 'linkerd-viz',
                version: '30.3.3',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
        },
    },
    argo: {
        repo: 'https://argoproj.github.io/argo-helm',
        charts: {
            argoCD: {
                chart: 'argo-cd',
                version: '5.5.12',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
        },
    },
}  satisfies ChartsInfo;

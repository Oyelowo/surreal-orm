type Repo = 'oyelowo' | 'nats' | 'pingcap' | 'bitnami' | 'longhorn' | 'jetstack' | 'linkerd' | 'argo' | 'meilisearch' | 'cilium' | 'grafana';

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
                version: '3.30',
                externalCrds: [] as string[],
                skipCrdRender: false
            },
        },
    },
    nats: {
        repo: 'https://nats-io.github.io/k8s/helm/charts',
        charts: {
            nats: {
                chart: 'nats',
                version: '0.18.1',
                externalCrds: [],
                skipCrdRender: false
            },
            // natsJetstream:
            nack: {
                chart: 'nack',
                version: '0.17.4',
                externalCrds: ['https://raw.githubusercontent.com/nats-io/nack/v0.6.0/deploy/crds.yml'],
                skipCrdRender: false
            },
            natsOperator: {
                chart: 'nats-operator',
                version: '0.7.4',
                externalCrds: [],
                skipCrdRender: false
            },
            natsAccountServer: {
                chart: 'nats-account-server',
                version: '0.8.0',
                externalCrds: [],
                skipCrdRender: false
            },
            natsKafka: {
                chart: 'nats-kafka',
                version: '0.13.1',
                externalCrds: [],
                skipCrdRender: false
            },
            surveyor: {
                chart: 'surveyor',
                version: '0.14.1',
                externalCrds: [],
                skipCrdRender: false
            },
        }
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
    longhorn: {
        repo: 'https://charts.longhorn.io',
        charts: {
            longhorn: {
                chart: 'longhorn',
                version: 'v1.3.2',
                externalCrds: [],
                skipCrdRender: false
            }
        }
    },
    bitnami: {
        repo: 'https://charts.bitnami.com/bitnami',
        charts: {
            sealedSecrets: {
                chart: 'sealed-secrets',
                version: '1.1.6',
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
            metalb: {
                chart: 'metallb',
                version: '4.1.5',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            redis: {
                chart: 'redis',
                version: '17.3.2',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            prometheus: {
                chart: 'kube-prometheus',
                version: '8.1.11',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            thanos: {
                chart: 'thanos',
                version: '11.5.5',
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
    meilisearch: {
        repo: 'https://meilisearch.github.io/meilisearch-kubernetes',
        charts: {
            meilisearch: {
                chart: 'meilisearch',
                version: '0.1.41',
                externalCrds: [] as string[],
                skipCrdRender: false
            }
        }
    },
    argo: {
        repo: 'https://argoproj.github.io/argo-helm',
        charts: {
            argoCD: {
                chart: 'argo-cd',
                version: '5.6.0',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            argoWorkflows: {
                chart: 'argo-workflows',
                version: '0.20.1',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            argoEvent: {
                chart: 'argo-events',
                version: '2.0.6',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            argoRollout: {
                chart: 'argo-rollouts',
                version: '2.21.1',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
            argoImageUpdater: {
                chart: 'argocd-image-updater',
                version: '0.8.1',
                externalCrds: [] as string[],
                skipCrdRender: false,
            },
        },
    },
    cilium: {
        repo: 'https://helm.cilium.io/',
        charts: {
            cilium: {
                chart: 'cilium',
                version: '1.12.3',
                externalCrds: [] as string[],
                skipCrdRender: false
            }
        }
    },
    grafana: {
        repo: 'https://grafana.github.io/helm-charts',
        charts: {
            grafana: {
                chart: 'grafana',
                version: '6.42.2',
                externalCrds: [] as string[],
                skipCrdRender: false
            },
            loki: {
                chart: 'loki-distributed',
                version: '0.63.1',
                externalCrds: [] as string[],
                skipCrdRender: false
            },
            tempo: {
                chart: 'tempo-distributed',
                version: '0.26.7',
                externalCrds: [] as string[],
                skipCrdRender: false
            },
            promtail: {
                chart: 'promtail',
                version: '6.5.1',
                externalCrds: [] as string[],
                skipCrdRender: false
            },
        }
    }
}  satisfies ChartsInfo;

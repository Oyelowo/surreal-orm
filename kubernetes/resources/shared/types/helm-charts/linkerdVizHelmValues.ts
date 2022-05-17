export interface LinkerdVizHelmValues {
    linkerdVersion: string
    clusterDomain: string
    identityTrustDomain: string
    defaultRegistry: string
    defaultImagePullPolicy: string
    defaultLogLevel: string
    defaultLogFormat: string
    defaultUID: number
    linkerdNamespace: string
    installNamespace: boolean
    namespace: string
    nodeSelector: NodeSelector
    imagePullSecrets: any[]
    tolerations: null
    enablePodAntiAffinity: boolean
    enablePSP: boolean
    prometheusUrl: string
    grafanaUrl: string
    jaegerUrl: string
    metricsAPI: Dashboard
    tap: Tap
    tapInjector: Tap
    dashboard: Dashboard
    grafana: Grafana
    prometheus: Prometheus
}

export interface Dashboard {
    replicas: number
    logLevel: string
    logFormat: string
    image: Image
    UID: null
    restrictPrivileges?: boolean
    enforcedHostRegexp?: string
    resources: Resources
    proxy: null
    nodeSelector?: NodeSelector
    tolerations?: any[]
}

export interface Image {
    registry: string
    name: string
    tag: string
    pullPolicy: string
}

export interface NodeSelector {
    'kubernetes.io/os': string
}

export interface Resources {
    cpu: CPU
    memory: CPU
}

export interface CPU {
    limit: null
    request: null
}

export interface Grafana {
    enabled: boolean
    logLevel: string
    logFormat: string
    image: Image
    resources: Resources
    proxy: null
    nodeSelector: NodeSelector
    tolerations: any[]
}

export interface Prometheus {
    enabled: boolean
    image: Image
    logLevel: string
    logFormat: string
    args: Args
    globalConfig: GlobalConfig
    alertRelabelConfigs: null
    alertmanagers: null
    remoteWrite: null
    ruleConfigMapMounts: null
    scrapeConfigs: null
    sidecarContainers: null
    resources: Resources
    proxy: null
    nodeSelector: NodeSelector
    tolerations: any[]
}

export interface Args {
    'storage.tsdb.path': string
    'storage.tsdb.retention.time': string
    'config.file': string
}

export interface GlobalConfig {
    scrape_interval: string
    scrape_timeout: string
    evaluation_interval: string
}

export interface Tap {
    replicas: number
    logLevel: string
    logFormat: string
    image: Image
    externalSecret: boolean
    crtPEM: string
    keyPEM: string
    caBundle: string
    resources: Resources
    proxy: null
    UID: null
    namespaceSelector?: null
    objectSelector?: null
    failurePolicy?: string
}

export interface ILinkerdvizlinkerd {
    linkerdVersion: string;
    clusterDomain: string;
    identityTrustDomain: string;
    defaultRegistry: string;
    defaultImagePullPolicy: string;
    defaultLogLevel: string;
    defaultLogFormat: string;
    defaultUID: number;
    linkerdNamespace: string;
    installNamespace: boolean;
    namespace: string;
    nodeSelector: NodeSelector;
    imagePullSecrets: any[];
    tolerations?: any;
    enablePodAntiAffinity: boolean;
    enablePSP: boolean;
    prometheusUrl: string;
    grafanaUrl: string;
    jaegerUrl: string;
    metricsAPI: MetricsAPI;
    tap: Tap;
    tapInjector: TapInjector;
    dashboard: Dashboard;
    grafana: Grafana;
    prometheus: Prometheus;
}
interface Prometheus {
    enabled: boolean;
    image: Image;
    logLevel: string;
    logFormat: string;
    args: Args;
    globalConfig: GlobalConfig;
    alertRelabelConfigs?: any;
    alertmanagers?: any;
    remoteWrite?: any;
    ruleConfigMapMounts?: any;
    scrapeConfigs?: any;
    sidecarContainers?: any;
    resources: Resources;
    proxy?: any;
    nodeSelector: NodeSelector;
    tolerations?: any;
}
interface GlobalConfig {
    scrape_interval: string;
    scrape_timeout: string;
    evaluation_interval: string;
}
interface Args {
    'storage.tsdb.path': string;
    'storage.tsdb.retention.time': string;
    'config.file': string;
}
interface Grafana {
    enabled: boolean;
    logLevel: string;
    logFormat: string;
    image: Image;
    resources: Resources;
    proxy?: any;
    nodeSelector: NodeSelector;
    tolerations?: any;
}
interface Dashboard {
    replicas: number;
    logLevel: string;
    logFormat: string;
    image: Image;
    UID?: any;
    restrictPrivileges: boolean;
    enforcedHostRegexp: string;
    resources: Resources;
    proxy?: any;
}
interface TapInjector {
    replicas: number;
    logLevel: string;
    logFormat: string;
    image: Image;
    namespaceSelector?: any;
    objectSelector?: any;
    UID?: any;
    failurePolicy: string;
    resources: Resources;
    proxy?: any;
    externalSecret: boolean;
    crtPEM: string;
    keyPEM: string;
    caBundle: string;
}
interface Tap {
    replicas: number;
    logLevel: string;
    logFormat: string;
    image: Image;
    externalSecret: boolean;
    crtPEM: string;
    keyPEM: string;
    caBundle: string;
    resources: Resources;
    proxy?: any;
    UID?: any;
}
interface MetricsAPI {
    replicas: number;
    logLevel: string;
    logFormat: string;
    image: Image;
    resources: Resources;
    proxy?: any;
    UID?: any;
    nodeSelector: NodeSelector;
    tolerations?: any;
}
interface Resources {
    cpu: Cpu;
    memory: Cpu;
}
interface Cpu {
    limit?: any;
    request?: any;
}
interface Image {
    registry: string;
    name: string;
    tag: string;
    pullPolicy: string;
}
interface NodeSelector {
    'kubernetes.io/os': string;
}

export interface ICertmanagertrustjetstack {
    replicaCount: number;
    image: Image;
    app: App;
    resources: Labels;
}
interface App {
    logLevel: number;
    metrics: Metrics;
    readinessProbe: ReadinessProbe;
    trust: Trust;
    webhook: Webhook;
}
interface Webhook {
    host: string;
    port: number;
    timeoutSeconds: number;
    service: Service2;
}
interface Service2 {
    type: string;
}
interface Trust {
    namespace: string;
}
interface ReadinessProbe {
    port: number;
    path: string;
}
interface Metrics {
    port: number;
    service: Service;
}
interface Service {
    enabled: boolean;
    type: string;
    servicemonitor: Servicemonitor;
}
interface Servicemonitor {
    enabled: boolean;
    prometheusInstance: string;
    interval: string;
    scrapeTimeout: string;
    labels: Labels;
}
interface Labels {}
interface Image {
    repository: string;
    tag: string;
    pullPolicy: string;
}

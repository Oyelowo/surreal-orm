export interface CertManagerTrustHelmValues {
    replicaCount: number
    image: Image
    app: App
    resources: Resources
}

export interface App {
    logLevel: number
    metrics: Metrics
    readinessProbe: ReadinessProbe
    trust: Trust
    webhook: Webhook
}

export interface Metrics {
    port: number
    service: MetricsService
}

export interface MetricsService {
    enabled: boolean
    type: string
    servicemonitor: Servicemonitor
}

export interface Servicemonitor {
    enabled: boolean
    prometheusInstance: string
    interval: string
    scrapeTimeout: string
    labels: Resources
}

export interface Resources {}

export interface ReadinessProbe {
    port: number
    path: string
}

export interface Trust {
    namespace: string
}

export interface Webhook {
    host: string
    port: number
    timeoutSeconds: number
    service: WebhookService
}

export interface WebhookService {
    type: string
}

export interface Image {
    repository: string
    tag: string
    pullPolicy: string
}

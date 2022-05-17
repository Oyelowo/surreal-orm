export interface CertManagerValuesBitnami {
    global: Global
    kubeVersion: string
    nameOverride: string
    fullnameOverride: string
    commonLabels: CommonAnnotations
    commonAnnotations: CommonAnnotations
    extraDeploy: any[]
    logLevel: number
    clusterResourceNamespace: string
    leaderElection: LeaderElection
    installCRDs: boolean
    replicaCount: number
    controller: Cainjector
    webhook: Cainjector
    cainjector: Cainjector
    metrics: Metrics
    rbac: Rbac
}

export interface Cainjector {
    replicaCount: number
    image: Image
    resources: Resources
    podSecurityContext: PodSecurityContext
    containerSecurityContext: ContainerSecurityContext
    podAffinityPreset: string
    podAntiAffinityPreset: string
    nodeAffinityPreset: NodeAffinityPreset
    affinity: CommonAnnotations
    nodeSelector: CommonAnnotations
    command: any[]
    args: any[]
    priorityClassName: string
    schedulerName: string
    topologySpreadConstraints: any[]
    hostAliases: any[]
    tolerations: any[]
    podLabels: CommonAnnotations
    podAnnotations: CommonAnnotations
    lifecycleHooks: CommonAnnotations
    updateStrategy: UpdateStrategy
    extraEnvVars: any[]
    extraEnvVarsCM: string
    extraEnvVarsSecret: string
    extraVolumes: any[]
    extraVolumeMounts: any[]
    initContainers: any[]
    sidecars: any[]
    serviceAccount: ServiceAccount
    acmesolver?: Acmesolver
    containerPort?: number
    dnsPolicy?: string
    dnsConfig?: CommonAnnotations
    httpsPort?: number
    livenessProbe?: NessProbe
    readinessProbe?: NessProbe
    customStartupProbe?: CommonAnnotations
    customLivenessProbe?: CommonAnnotations
    customReadinessProbe?: CommonAnnotations
}

export interface Acmesolver {
    image: Image
}

export interface Image {
    registry: string
    repository: string
    tag: string
    pullPolicy: string
    pullSecrets: any[]
    debug: boolean
}

export interface CommonAnnotations {}

export interface ContainerSecurityContext {
    enabled: boolean
    runAsUser: number
    runAsNonRoot: boolean
}

export interface NessProbe {
    enabled: boolean
    path: string
    initialDelaySeconds: number
    periodSeconds: number
    timeoutSeconds: number
    successThreshold: number
    failureThreshold: number
}

export interface NodeAffinityPreset {
    type: string
    key: string
    values: any[]
}

export interface PodSecurityContext {
    enabled: boolean
    fsGroup: number
}

export interface Resources {
    limits: CommonAnnotations
    requests: CommonAnnotations
}

export interface ServiceAccount {
    create: boolean
    name: string
    annotations: CommonAnnotations
    automountServiceAccountToken: boolean
}

export interface UpdateStrategy {
    type: string
    rollingUpdate: CommonAnnotations
}

export interface Global {
    imageRegistry: string
    imagePullSecrets: any[]
    storageClass: string
}

export interface LeaderElection {
    namespace: string
}

export interface Metrics {
    enabled: boolean
    podAnnotations: PodAnnotations
    serviceMonitor: ServiceMonitor
}

export interface PodAnnotations {
    'prometheus.io/path': string
    'prometheus.io/scrape': string
    'prometheus.io/port': string
}

export interface ServiceMonitor {
    path: string
    targetPort: number
    enabled: boolean
    namespace: string
    jobLabel: string
    interval: string
    scrapeTimeout: string
    relabelings: any[]
    metricRelabelings: any[]
    selector: CommonAnnotations
    labels: CommonAnnotations
    additionalLabels: CommonAnnotations
    honorLabels: boolean
}

export interface Rbac {
    create: boolean
}

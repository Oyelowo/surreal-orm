export interface ArgocdHelmValuesBitnami {
    global: Global
    kubeVersion: string
    nameOverride: string
    fullnameOverride: string
    commonLabels: CommonAnnotations
    commonAnnotations: CommonAnnotations
    clusterDomain: string
    extraDeploy: any[]
    image: Image
    controller: Controller
    server: Server
    repoServer: RepoServer
    dex: Dex
    config: Welcome5Config
    volumePermissions: VolumePermissions
    rbac: Rbac
    redis: Redis
    externalRedis: ExternalRedis
}

export interface CommonAnnotations {}

export interface Welcome5Config {
    knownHosts: string
    extraKnownHosts: string
    createExtraKnownHosts: boolean
    styles: string
    existingStylesConfigmap: string
    tlsCerts: CommonAnnotations
    secret: Secret
    clusterCredentials: any[]
}

export interface Secret {
    create: boolean
    annotations: CommonAnnotations
    githubSecret: string
    gitlabSecret: string
    bitbucketServerSecret: string
    bitbucketUUID: string
    gogsSecret: string
    extra: CommonAnnotations
    argocdServerTlsConfig: ArgocdServerTLSConfig
    argocdServerAdminPassword: string
    argocdServerAdminPasswordMtime: string
    repositoryCredentials: CommonAnnotations
}

export interface ArgocdServerTLSConfig {
    key: string
    crt: string
}

export interface Controller {
    replicaCount: number
    startupProbe: Probe
    livenessProbe: Probe
    readinessProbe: Probe
    customStartupProbe: CommonAnnotations
    customLivenessProbe: CommonAnnotations
    customReadinessProbe: CommonAnnotations
    resources: Resources
    podSecurityContext: PodSecurityContext
    containerSecurityContext: ControllerContainerSecurityContext
    serviceAccount: ServiceAccount
    clusterAdminAccess: boolean
    clusterRoleRules: any[]
    logFormat: string
    logLevel: string
    containerPorts: ControllerContainerPorts
    service: MetricsService
    metrics: Metrics
    command: any[]
    defaultArgs: DefaultArgs
    args: any[]
    extraArgs: any[]
    hostAliases: any[]
    podLabels: CommonAnnotations
    podAnnotations: CommonAnnotations
    podAffinityPreset: string
    podAntiAffinityPreset: string
    nodeAffinityPreset: NodeAffinityPreset
    affinity: CommonAnnotations
    nodeSelector: CommonAnnotations
    tolerations: any[]
    schedulerName: string
    topologySpreadConstraints: any[]
    updateStrategy: UpdateStrategy
    priorityClassName: string
    lifecycleHooks: CommonAnnotations
    extraEnvVars: any[]
    extraEnvVarsCM: string
    extraEnvVarsSecret: string
    extraVolumes: any[]
    extraVolumeMounts: any[]
    sidecars: any[]
    initContainers: any[]
}

export interface ControllerContainerPorts {
    controller: number
    metrics: number
}

export interface ControllerContainerSecurityContext {
    enabled: boolean
    runAsUser: number
    runAsNonRoot: boolean
}

export interface DefaultArgs {
    statusProcessors: string
    operationProcessors: string
    appResyncPeriod: string
    selfHealTimeout: string
}

export interface Probe {
    enabled: boolean
    initialDelaySeconds: number
    periodSeconds: number
    timeoutSeconds: number
    failureThreshold: number
    successThreshold: number
}

export interface Metrics {
    enabled: boolean
    service: MetricsService
    serviceMonitor: ServiceMonitor
    rules?: Rules
}

export interface Rules {
    enabled: boolean
    spec: any[]
    selector: CommonAnnotations
    namespace: string
    additionalLabels: CommonAnnotations
}

export interface MetricsService {
    type: string
    port?: number
    nodePort?: string
    clusterIP: string
    loadBalancerIP: string
    loadBalancerSourceRanges: any[]
    externalTrafficPolicy: string
    annotations: CommonAnnotations
    sessionAffinity: string
    sessionAffinityConfig: CommonAnnotations
    extraPorts?: any[]
    ports?: Ports
    nodePorts?: NodePorts
}

export interface NodePorts {
    http: string
    grpc?: string
    https?: string
}

export interface Ports {
    http: number
    grpc?: number
    https?: number
}

export interface ServiceMonitor {
    enabled: boolean
    namespace: string
    jobLabel: string
    interval: string
    scrapeTimeout: string
    relabelings: any[]
    metricRelabelings: any[]
    selector: CommonAnnotations
    honorLabels: boolean
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
    automountServiceAccountToken: boolean
    annotations: CommonAnnotations
}

export interface UpdateStrategy {
    type: string
}

export interface Dex {
    image: Image
    enabled: boolean
    replicaCount: number
    startupProbe: Probe
    livenessProbe: Probe
    readinessProbe: Probe
    customStartupProbe: CommonAnnotations
    customLivenessProbe: CommonAnnotations
    customReadinessProbe: CommonAnnotations
    resources: Resources
    podSecurityContext: PodSecurityContext
    containerSecurityContext: ControllerContainerSecurityContext
    service: MetricsService
    containerPorts: DexContainerPorts
    metrics: Metrics
    serviceAccount: ServiceAccount
    command: any[]
    args: any[]
    extraArgs: any[]
    hostAliases: any[]
    podLabels: CommonAnnotations
    podAnnotations: CommonAnnotations
    podAffinityPreset: string
    podAntiAffinityPreset: string
    nodeAffinityPreset: NodeAffinityPreset
    affinity: CommonAnnotations
    nodeSelector: CommonAnnotations
    tolerations: any[]
    schedulerName: string
    topologySpreadConstraints: any[]
    updateStrategy: UpdateStrategy
    priorityClassName: string
    lifecycleHooks: CommonAnnotations
    extraEnvVars: any[]
    extraEnvVarsCM: string
    extraEnvVarsSecret: string
    extraVolumes: any[]
    extraVolumeMounts: any[]
    sidecars: any[]
    initContainers: any[]
}

export interface DexContainerPorts {
    http: number
    grpc: number
    metrics: number
}

export interface Image {
    registry: string
    repository: string
    tag: string
    pullPolicy: string
    pullSecrets: any[]
    debug?: boolean
}

export interface ExternalRedis {
    host: string
    port: number
    password: string
    existingSecret: string
    existingSecretPasswordKey: string
}

export interface Global {
    imageRegistry: string
    imagePullSecrets: any[]
    storageClass: string
}

export interface Rbac {
    create: boolean
}

export interface Redis {
    image: Image
    enabled: boolean
    nameOverride: string
    service: RedisService
    auth: Auth
    architecture: string
}

export interface Auth {
    enabled: boolean
    existingSecret: string
    existingSecretPasswordKey: string
}

export interface RedisService {
    port: number
}

export interface RepoServer {
    replicaCount: number
    startupProbe: Probe
    livenessProbe: Probe
    readinessProbe: Probe
    customStartupProbe: CommonAnnotations
    customLivenessProbe: CommonAnnotations
    customReadinessProbe: CommonAnnotations
    resources: Resources
    podSecurityContext: PodSecurityContext
    containerSecurityContext: ControllerContainerSecurityContext
    service: MetricsService
    logFormat: string
    logLevel: string
    containerPorts: RepoServerContainerPorts
    metrics: Metrics
    autoscaling: Autoscaling
    serviceAccount: ServiceAccount
    command: any[]
    args: any[]
    extraArgs: any[]
    hostAliases: any[]
    podLabels: CommonAnnotations
    podAnnotations: CommonAnnotations
    podAffinityPreset: string
    podAntiAffinityPreset: string
    nodeAffinityPreset: NodeAffinityPreset
    affinity: CommonAnnotations
    nodeSelector: CommonAnnotations
    tolerations: any[]
    schedulerName: string
    topologySpreadConstraints: any[]
    updateStrategy: UpdateStrategy
    priorityClassName: string
    lifecycleHooks: CommonAnnotations
    extraEnvVars: any[]
    extraEnvVarsCM: string
    extraEnvVarsSecret: string
    extraVolumes: any[]
    extraVolumeMounts: any[]
    sidecars: any[]
    initContainers: any[]
}

export interface Autoscaling {
    enabled: boolean
    minReplicas: number
    maxReplicas: number
    targetCPU: number
    targetMemory: number
}

export interface RepoServerContainerPorts {
    repoServer: number
    metrics: string
}

export interface Server {
    replicaCount: number
    startupProbe: Probe
    livenessProbe: Probe
    readinessProbe: Probe
    customStartupProbe: CommonAnnotations
    customLivenessProbe: CommonAnnotations
    customReadinessProbe: CommonAnnotations
    resources: Resources
    podSecurityContext: PodSecurityContext
    containerSecurityContext: ControllerContainerSecurityContext
    autoscaling: Autoscaling
    insecure: boolean
    logFormat: string
    logLevel: string
    configEnabled: boolean
    url: string
    config: ServerConfig
    ingress: Ingress
    metrics: Metrics
    ingressGrpc: Ingress
    containerPorts: ServerContainerPorts
    service: MetricsService
    command: any[]
    args: any[]
    extraArgs: any[]
    hostAliases: any[]
    podLabels: CommonAnnotations
    podAnnotations: CommonAnnotations
    podAffinityPreset: string
    podAntiAffinityPreset: string
    nodeAffinityPreset: NodeAffinityPreset
    affinity: CommonAnnotations
    nodeSelector: CommonAnnotations
    tolerations: any[]
    schedulerName: string
    topologySpreadConstraints: any[]
    updateStrategy: UpdateStrategy
    priorityClassName: string
    lifecycleHooks: CommonAnnotations
    extraEnvVars: any[]
    extraEnvVarsCM: string
    extraEnvVarsSecret: string
    extraVolumes: any[]
    extraVolumeMounts: any[]
    sidecars: any[]
    initContainers: any[]
    serviceAccount: ServiceAccount
}

export interface ServerConfig {
    url: string
    'application.instanceLabelKey': string
    'dex.config': string
}

export interface ServerContainerPorts {
    http: number
    https: number
    metrics: number
}

export interface Ingress {
    enabled: boolean
    selfSigned: boolean
    pathType: string
    apiVersion: string
    hostname: string
    path: string
    annotations: CommonAnnotations
    tls: boolean
    extraHosts: any[]
    extraPaths: any[]
    extraTls: any[]
    secrets: any[]
    ingressClassName: string
}

export interface VolumePermissions {
    enabled: boolean
    image: Image
    resources: Resources
    containerSecurityContext: VolumePermissionsContainerSecurityContext
}

export interface VolumePermissionsContainerSecurityContext {
    runAsUser: number
}

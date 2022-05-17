export interface RedisHelmValuesBitnami {
    global: Global
    kubeVersion: string
    nameOverride: string
    fullnameOverride: string
    commonLabels: CommonAnnotations
    commonAnnotations: CommonAnnotations
    clusterDomain: string
    extraDeploy: any[]
    diagnosticMode: DiagnosticMode
    image: Image
    architecture: string
    auth: Auth
    commonConfiguration: string
    existingConfigmap: string
    master: Master
    replica: Replica
    sentinel: Sentinel
    networkPolicy: NetworkPolicy
    podSecurityPolicy: PodSecurityPolicy
    rbac: Rbac
    serviceAccount: ServiceAccount
    pdb: Pdb
    tls: TLS
    metrics: Metrics
    volumePermissions: VolumePermissions
    sysctl: Sysctl
    useExternalDNS: UseExternalDNS
}

export interface Auth {
    enabled: boolean
    sentinel: boolean
    password: string
    existingSecret: string
    existingSecretPasswordKey: string
    usePasswordFiles: boolean
}

export interface CommonAnnotations {}

export interface DiagnosticMode {
    enabled: boolean
    command: string[]
    args: string[]
}

export interface Global {
    imageRegistry: string
    imagePullSecrets: any[]
    storageClass: string
    redis: Redis
}

export interface Redis {
    password: string
}

export interface Image {
    registry: string
    repository: string
    tag: string
    pullPolicy: string
    pullSecrets: any[]
    debug?: boolean
}

export interface Master {
    configuration: string
    disableCommands: string[]
    command: any[]
    args: any[]
    preExecCmds: any[]
    extraFlags: any[]
    extraEnvVars: any[]
    extraEnvVarsCM: string
    extraEnvVarsSecret: string
    containerPorts: ContainerPortsClass
    startupProbe: Probe
    livenessProbe: Probe
    readinessProbe: Probe
    customStartupProbe: CommonAnnotations
    customLivenessProbe: CommonAnnotations
    customReadinessProbe: CommonAnnotations
    resources: Resources
    podSecurityContext: PodSecurityContext
    containerSecurityContext: MasterContainerSecurityContext
    kind: string
    schedulerName: string
    updateStrategy: UpdateStrategy
    priorityClassName: string
    hostAliases: any[]
    podLabels: CommonAnnotations
    podAnnotations: CommonAnnotations
    shareProcessNamespace: boolean
    podAffinityPreset: string
    podAntiAffinityPreset: string
    nodeAffinityPreset: NodeAffinityPreset
    affinity: CommonAnnotations
    nodeSelector: CommonAnnotations
    tolerations: any[]
    topologySpreadConstraints: CommonAnnotations
    lifecycleHooks: CommonAnnotations
    extraVolumes: any[]
    extraVolumeMounts: any[]
    sidecars: any[]
    initContainers: any[]
    persistence: MasterPersistence
    service: MasterService
    terminationGracePeriodSeconds: number
}

export interface ContainerPortsClass {
    redis: number
}

export interface MasterContainerSecurityContext {
    enabled: boolean
    runAsUser: number
}

export interface Probe {
    enabled: boolean
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

export interface MasterPersistence {
    enabled: boolean
    medium: string
    path: string
    subPath: string
    storageClass: string
    accessModes: string[]
    size: string
    annotations: CommonAnnotations
    selector: CommonAnnotations
    dataSource: CommonAnnotations
    existingClaim?: string
}

export interface PodSecurityContext {
    enabled: boolean
    fsGroup: number
}

export interface Resources {
    limits: CommonAnnotations
    requests: CommonAnnotations
}

export interface MasterService {
    type: string
    ports: ContainerPortsClass
    nodePorts: PurpleNodePorts
    externalTrafficPolicy: string
    extraPorts: any[]
    clusterIP: string
    loadBalancerIP: string
    loadBalancerSourceRanges: any[]
    annotations: CommonAnnotations
}

export interface PurpleNodePorts {
    redis: string
}

export interface UpdateStrategy {
    type: string
    rollingUpdate: CommonAnnotations
}

export interface Metrics {
    enabled: boolean
    image: Image
    command: any[]
    redisTargetHost: string
    extraArgs: CommonAnnotations
    containerSecurityContext: MasterContainerSecurityContext
    extraVolumes: any[]
    extraVolumeMounts: any[]
    resources: Resources
    podLabels: CommonAnnotations
    podAnnotations: PodAnnotations
    service: MetricsService
    serviceMonitor: ServiceMonitor
    prometheusRule: PrometheusRule
}

export interface PodAnnotations {
    'prometheus.io/scrape': string
    'prometheus.io/port': string
}

export interface PrometheusRule {
    enabled: boolean
    namespace: string
    additionalLabels: CommonAnnotations
    rules: any[]
}

export interface MetricsService {
    type: string
    port: number
    externalTrafficPolicy: string
    extraPorts: any[]
    loadBalancerIP: string
    loadBalancerSourceRanges: any[]
    annotations: CommonAnnotations
}

export interface ServiceMonitor {
    enabled: boolean
    namespace: string
    interval: string
    scrapeTimeout: string
    relabellings: any[]
    metricRelabelings: any[]
    honorLabels: boolean
    additionalLabels: CommonAnnotations
}

export interface NetworkPolicy {
    enabled: boolean
    allowExternal: boolean
    extraIngress: any[]
    extraEgress: any[]
    ingressNSMatchLabels: CommonAnnotations
    ingressNSPodMatchLabels: CommonAnnotations
}

export interface Pdb {
    create: boolean
    minAvailable: number
    maxUnavailable: string
}

export interface PodSecurityPolicy {
    create: boolean
    enabled: boolean
}

export interface Rbac {
    create: boolean
    rules: any[]
}

export interface Replica {
    replicaCount: number
    configuration: string
    disableCommands: string[]
    command: any[]
    args: any[]
    preExecCmds: any[]
    extraFlags: any[]
    extraEnvVars: any[]
    extraEnvVarsCM: string
    extraEnvVarsSecret: string
    externalMaster: ExternalMaster
    containerPorts: ContainerPortsClass
    startupProbe: Probe
    livenessProbe: Probe
    readinessProbe: Probe
    customStartupProbe: CommonAnnotations
    customLivenessProbe: CommonAnnotations
    customReadinessProbe: CommonAnnotations
    resources: Resources
    podSecurityContext: PodSecurityContext
    containerSecurityContext: MasterContainerSecurityContext
    schedulerName: string
    updateStrategy: UpdateStrategy
    priorityClassName: string
    podManagementPolicy: string
    hostAliases: any[]
    podLabels: CommonAnnotations
    podAnnotations: CommonAnnotations
    shareProcessNamespace: boolean
    podAffinityPreset: string
    podAntiAffinityPreset: string
    nodeAffinityPreset: NodeAffinityPreset
    affinity: CommonAnnotations
    nodeSelector: CommonAnnotations
    tolerations: any[]
    topologySpreadConstraints: CommonAnnotations
    lifecycleHooks: CommonAnnotations
    extraVolumes: any[]
    extraVolumeMounts: any[]
    sidecars: any[]
    initContainers: any[]
    persistence: MasterPersistence
    service: MasterService
    terminationGracePeriodSeconds: number
    autoscaling: Autoscaling
}

export interface Autoscaling {
    enabled: boolean
    minReplicas: number
    maxReplicas: number
    targetCPU: string
    targetMemory: string
}

export interface ExternalMaster {
    enabled: boolean
    host: string
    port: number
}

export interface Sentinel {
    enabled: boolean
    image: Image
    masterSet: string
    quorum: number
    automateClusterRecovery: boolean
    downAfterMilliseconds: number
    failoverTimeout: number
    parallelSyncs: number
    configuration: string
    command: any[]
    args: any[]
    preExecCmds: any[]
    extraEnvVars: any[]
    extraEnvVarsCM: string
    extraEnvVarsSecret: string
    externalMaster: ExternalMaster
    containerPorts: ContainerPorts
    startupProbe: Probe
    livenessProbe: Probe
    readinessProbe: Probe
    customStartupProbe: CommonAnnotations
    customLivenessProbe: CommonAnnotations
    customReadinessProbe: CommonAnnotations
    persistence: SentinelPersistence
    resources: Resources
    containerSecurityContext: MasterContainerSecurityContext
    lifecycleHooks: CommonAnnotations
    extraVolumes: any[]
    extraVolumeMounts: any[]
    service: SentinelService
    terminationGracePeriodSeconds: number
}

export interface ContainerPorts {
    sentinel: number
}

export interface SentinelPersistence {
    enabled: boolean
    storageClass: string
    accessModes: string[]
    size: string
    annotations: CommonAnnotations
    selector: CommonAnnotations
    dataSource: CommonAnnotations
}

export interface SentinelService {
    type: string
    ports: PurplePorts
    nodePorts: FluffyNodePorts
    externalTrafficPolicy: string
    extraPorts: any[]
    clusterIP: string
    loadBalancerIP: string
    loadBalancerSourceRanges: any[]
    annotations: CommonAnnotations
}

export interface FluffyNodePorts {
    redis: string
    sentinel: string
}

export interface PurplePorts {
    redis: number
    sentinel: number
}

export interface ServiceAccount {
    create: boolean
    name: string
    automountServiceAccountToken: boolean
    annotations: CommonAnnotations
}

export interface Sysctl {
    enabled: boolean
    image: Image
    command: any[]
    mountHostSys: boolean
    resources: Resources
}

export interface TLS {
    enabled: boolean
    authClients: boolean
    autoGenerated: boolean
    existingSecret: string
    certificatesSecret: string
    certFilename: string
    certKeyFilename: string
    certCAFilename: string
    dhParamsFilename: string
}

export interface UseExternalDNS {
    enabled: boolean
    suffix: string
    annotationKey: string
    additionalAnnotations: CommonAnnotations
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

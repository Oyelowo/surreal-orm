export interface postgresdbHaHelmValuesBitnami {
    global: Global;
    nameOverride: string;
    fullnameOverride: string;
    clusterDomain: string;
    commonAnnotations: CommonAnnotations;
    commonLabels: CommonAnnotations;
    extraDeploy: any[];
    serviceAccount: ServiceAccount;
    psp: Psp;
    rbac: Psp;
    topologySpreadConstraints: CommonAnnotations;
    diagnosticMode: DiagnosticMode;
    postgresqlImage: Image;
    postgresql: Welcome9Postgresql;
    pgpoolImage: Image;
    pgpool: Welcome9Pgpool;
    ldap: Welcome9LDAP;
    metricsImage: Image;
    metrics: Metrics;
    volumePermissionsImage: Image;
    volumePermissions: VolumePermissions;
    persistence: Persistence;
    service: Welcome9Service;
    networkPolicy: NetworkPolicy;
}

export interface CommonAnnotations {}

export interface DiagnosticMode {
    enabled: boolean;
    command: string[];
    args: string[];
}

export interface Global {
    imageRegistry: string;
    imagePullSecrets: any[];
    storageClass: string;
    postgresql: GlobalPostgresql;
    ldap: GlobalLDAP;
    pgpool: GlobalPgpool;
}

export interface GlobalLDAP {
    bindpw: string;
    existingSecret: string;
}

export interface GlobalPgpool {
    adminUsername: string;
    adminPassword: string;
    existingSecret: string;
}

export interface GlobalPostgresql {
    username: string;
    password: string;
    database: string;
    repmgrUsername: string;
    repmgrPassword: string;
    repmgrDatabase: string;
    existingSecret: string;
}

export interface Welcome9LDAP {
    enabled: boolean;
    existingSecret: string;
    uri: string;
    base: string;
    binddn: string;
    bindpw: string;
    bslookup: string;
    scope: string;
    tlsReqcert: string;
    nssInitgroupsIgnoreusers: string;
}

export interface Metrics {
    enabled: boolean;
    securityContext: SecurityContext;
    resources: Resources;
    containerPort: number;
    livenessProbe: Probe;
    readinessProbe: Probe;
    startupProbe: Probe;
    service: MetricsService;
    annotations: Annotations;
    customMetrics: CommonAnnotations;
    extraEnvVars: CommonAnnotations;
    serviceMonitor: ServiceMonitor;
}

export interface Annotations {
    'prometheus.io/scrape': string;
    'prometheus.io/port': string;
}

export interface Probe {
    enabled: boolean;
    initialDelaySeconds: number;
    periodSeconds: number;
    timeoutSeconds: number;
    successThreshold: number;
    failureThreshold: number;
}

export interface Resources {
    limits: CommonAnnotations;
    requests: CommonAnnotations;
}

export interface SecurityContext {
    enabled: boolean;
    runAsUser: number;
}

export interface MetricsService {
    type: string;
    port: number;
    nodePort: string;
    clusterIP: string;
    loadBalancerIP: string;
    loadBalancerSourceRanges: any[];
    externalTrafficPolicy: string;
}

export interface ServiceMonitor {
    enabled: boolean;
    namespace: string;
    interval: string;
    scrapeTimeout: string;
    selector: Selector;
    relabelings: any[];
    metricRelabelings: any[];
}

export interface Selector {
    prometheus: string;
}

export interface Image {
    registry: string;
    repository: string;
    tag: string;
    pullPolicy: string;
    pullSecrets: any[];
    debug?: boolean;
}

export interface NetworkPolicy {
    enabled: boolean;
    allowExternal: boolean;
    egressRules: EgressRules;
}

export interface EgressRules {
    denyConnectionsToExternal: boolean;
    customRules: any[];
}

export interface Persistence {
    enabled: boolean;
    existingClaim: string;
    storageClass: string;
    mountPath: string;
    accessModes: string[];
    size: string;
    annotations: CommonAnnotations;
    selector: CommonAnnotations;
}

export interface Welcome9Pgpool {
    customUsers: CommonAnnotations;
    usernames: string;
    passwords: string;
    hostAliases: any[];
    customUsersSecret: string;
    existingSecret: string;
    srCheckDatabase: string;
    labels: CommonAnnotations;
    podLabels: CommonAnnotations;
    serviceLabels: CommonAnnotations;
    customLivenessProbe: CommonAnnotations;
    customReadinessProbe: CommonAnnotations;
    customStartupProbe: CommonAnnotations;
    command: any[];
    args: any[];
    lifecycleHooks: CommonAnnotations;
    extraEnvVars: any[];
    extraEnvVarsCM: string;
    extraEnvVarsSecret: string;
    extraVolumes: any[];
    extraVolumeMounts: any[];
    initContainers: any[];
    sidecars: any[];
    replicaCount: number;
    podAnnotations: CommonAnnotations;
    priorityClassName: string;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: CommonAnnotations;
    nodeSelector: CommonAnnotations;
    tolerations: any[];
    securityContext: PurpleSecurityContext;
    containerSecurityContext: SecurityContext;
    resources: Resources;
    livenessProbe: Probe;
    readinessProbe: Probe;
    startupProbe: Probe;
    pdb: Pdb;
    updateStrategy: CommonAnnotations;
    containerPort: number;
    minReadySeconds: string;
    adminUsername: string;
    adminPassword: string;
    logConnections: boolean;
    logHostname: boolean;
    logPerNodeStatement: boolean;
    logLinePrefix: string;
    clientMinMessages: string;
    numInitChildren: string;
    reservedConnections: number;
    maxPool: string;
    childMaxConnections: string;
    childLifeTime: string;
    clientIdleLimit: string;
    connectionLifeTime: string;
    useLoadBalancing: boolean;
    loadBalancingOnWrite: string;
    configuration: string;
    configurationCM: string;
    initdbScripts: CommonAnnotations;
    initdbScriptsCM: string;
    initdbScriptsSecret: string;
    tls: TLS;
}

export interface NodeAffinityPreset {
    type: string;
    key: string;
    values: any[];
}

export interface Pdb {
    create: boolean;
    minAvailable: number;
    maxUnavailable: string;
}

export interface PurpleSecurityContext {
    enabled: boolean;
    fsGroup: number;
}

export interface TLS {
    enabled: boolean;
    autoGenerated?: boolean;
    preferServerCiphers: boolean;
    certificatesSecret: string;
    certFilename: string;
    certKeyFilename: string;
    certCAFilename: string;
}

export interface Welcome9Postgresql {
    labels: CommonAnnotations;
    podLabels: CommonAnnotations;
    replicaCount: number;
    updateStrategyType: string;
    containerPort: number;
    hostAliases: any[];
    podAnnotations: CommonAnnotations;
    priorityClassName: string;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: CommonAnnotations;
    nodeSelector: CommonAnnotations;
    tolerations: any[];
    securityContext: PurpleSecurityContext;
    containerSecurityContext: SecurityContext;
    customLivenessProbe: CommonAnnotations;
    customReadinessProbe: CommonAnnotations;
    customStartupProbe: CommonAnnotations;
    command: any[];
    args: any[];
    lifecycleHooks: CommonAnnotations;
    extraEnvVars: any[];
    extraEnvVarsCM: string;
    extraEnvVarsSecret: string;
    extraVolumes: any[];
    extraVolumeMounts: any[];
    initContainers: any[];
    sidecars: any[];
    resources: Resources;
    livenessProbe: Probe;
    readinessProbe: Probe;
    startupProbe: Probe;
    pdb: Pdb;
    username: string;
    password: string;
    database: string;
    existingSecret: string;
    postgresPassword: string;
    usePasswordFile: string;
    repmgrUsePassfile: string;
    repmgrPassfilePath: string;
    upgradeRepmgrExtension: boolean;
    pgHbaTrustAll: boolean;
    syncReplication: boolean;
    repmgrUsername: string;
    repmgrPassword: string;
    repmgrDatabase: string;
    repmgrLogLevel: string;
    repmgrConnectTimeout: number;
    repmgrReconnectAttempts: number;
    repmgrReconnectInterval: number;
    usePgRewind: boolean;
    audit: Audit;
    sharedPreloadLibraries: string;
    maxConnections: string;
    postgresConnectionLimit: string;
    dbUserConnectionLimit: string;
    tcpKeepalivesInterval: string;
    tcpKeepalivesIdle: string;
    tcpKeepalivesCount: string;
    statementTimeout: string;
    pghbaRemoveFilters: string;
    extraInitContainers: any[];
    repmgrConfiguration: string;
    configuration: string;
    pgHbaConfiguration: string;
    configurationCM: string;
    extendedConf: string;
    extendedConfCM: string;
    initdbScripts: CommonAnnotations;
    initdbScriptsCM: string;
    initdbScriptsSecret: string;
    tls: TLS;
}

export interface Audit {
    logHostname: boolean;
    logConnections: boolean;
    logDisconnections: boolean;
    pgAuditLog: string;
    pgAuditLogCatalog: string;
    clientMinMessages: string;
    logLinePrefix: string;
    logTimezone: string;
}

export interface Psp {
    create: boolean;
}

export interface Welcome9Service {
    type: string;
    port: number;
    portName: string;
    nodePort: string;
    loadBalancerIP: string;
    loadBalancerSourceRanges: any[];
    clusterIP: string;
    externalTrafficPolicy: string;
    sessionAffinity: string;
    annotations: CommonAnnotations;
    serviceLabels: CommonAnnotations;
}

export interface ServiceAccount {
    enabled: boolean;
    name: string;
}

export interface VolumePermissions {
    enabled: boolean;
    securityContext: VolumePermissionsSecurityContext;
    resources: Resources;
}

export interface VolumePermissionsSecurityContext {
    runAsUser: number;
}

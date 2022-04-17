export interface ArgocdHelmValuesArgo {
    nameOverride: string;
    fullnameOverride: string;
    kubeVersionOverride: string;
    global: Global;
    apiVersionOverrides: APIVersionOverrides;
    createAggregateRoles: boolean;
    extraObjects: any[];
    controller: Controller;
    dex: Dex;
    redis: Welcome3Redis;
    "redis-ha": RedisHa;
    externalRedis: ExternalRedis;
    server: Server;
    repoServer: RepoServer;
    configs: Configs;
    openshift: Openshift;
    applicationSet: ApplicationSet;
    notifications: Notifications;
}

export interface APIVersionOverrides {
    certmanager: string;
    ingress: string;
}

export interface ApplicationSet {
    enabled: boolean;
    name: string;
    replicaCount: number;
    image: ApplicationSetImage;
    args: ApplicationSetArgs;
    extraContainers: any[];
    metrics: ApplicationSetMetrics;
    imagePullSecrets: any[];
    service: ApplicationSetService;
    serviceAccount: ServiceAccount;
    podAnnotations: Affinity;
    podLabels: Affinity;
    podSecurityContext: Affinity;
    securityContext: Affinity;
    resources: Affinity;
    nodeSelector: Affinity;
    tolerations: any[];
    affinity: Affinity;
    priorityClassName: string;
    extraVolumeMounts: any[];
    extraVolumes: any[];
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    webhook: Webhook;
}

export interface Affinity {
}

export interface ApplicationSetArgs {
    metricsAddr: string;
    probeBindAddr: string;
    enableLeaderElection: boolean;
    policy: string;
    debug: boolean;
    dryRun: boolean;
}

export interface ApplicationSetImage {
    repository: string;
    pullPolicy: PullPolicy;
    tag: string;
}

export enum PullPolicy {
    Empty = "",
    IfNotPresent = "IfNotPresent",
}

export interface ApplicationSetMetrics {
    enabled: boolean;
    service: PurpleService;
    serviceMonitor: PurpleServiceMonitor;
}

export interface PurpleService {
    annotations: Affinity;
    labels: Affinity;
    servicePort: number;
}

export interface PurpleServiceMonitor {
    enabled: boolean;
    interval: string;
    relabelings: any[];
    metricRelabelings: any[];
    selector: Affinity;
    namespace: string;
    additionalLabels: Affinity;
}

export interface ApplicationSetService {
    annotations: Affinity;
    labels: Affinity;
    port: number;
    portName: string;
}

export interface ServiceAccount {
    create: boolean;
    annotations: Affinity;
    name: string;
    automountServiceAccountToken?: boolean;
    items?: Affinity;
}

export interface Webhook {
    ingress: Ingress;
}

export interface Ingress {
    enabled: boolean;
    annotations: Affinity;
    labels: Affinity;
    ingressClassName: string;
    hosts: any[];
    paths: string[];
    pathType: string;
    extraPaths: any[];
    tls: any[];
    https?: boolean;
    isAWSALB?: boolean;
    awsALB?: AwsALB;
}

export interface AwsALB {
    serviceType: string;
    backendProtocolVersion: string;
}

export interface Configs {
    clusterCredentials: any[];
    gpgKeysAnnotations: Affinity;
    gpgKeys: Affinity;
    knownHostsAnnotations: Affinity;
    knownHosts: KnownHosts;
    tlsCertsAnnotations: Affinity;
    tlsCerts: Affinity;
    repositoryCredentials: Affinity;
    credentialTemplates: Affinity;
    repositories: Affinity;
    secret: Secret;
    styles: string;
}

export interface KnownHosts {
    data: Data;
}

export interface Data {
    ssh_known_hosts: string;
}

export interface Secret {
    createSecret: boolean;
    annotations: Affinity;
    githubSecret: string;
    gitlabSecret: string;
    bitbucketServerSecret: string;
    bitbucketUUID: string;
    gogsSecret: string;
    extra: Affinity;
    argocdServerTlsConfig: Affinity;
    argocdServerAdminPassword: string;
    argocdServerAdminPasswordMtime: string;
}

export interface Controller {
    name: string;
    image: InitImageClass;
    replicas: number;
    enableStatefulSet: boolean;
    args: ControllerArgs;
    logFormat: string;
    logLevel: string;
    extraArgs: any[];
    env: any[];
    envFrom: any[];
    podAnnotations: Affinity;
    podLabels: Affinity;
    containerSecurityContext: Affinity;
    containerPort: number;
    readinessProbe: NessProbe;
    livenessProbe: NessProbe;
    volumeMounts: any[];
    volumes: any[];
    service: ApplicationSetService;
    nodeSelector: Affinity;
    tolerations: any[];
    affinity: Affinity;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    resources: Affinity;
    serviceAccount: ServiceAccount;
    metrics: ControllerMetrics;
    clusterAdminAccess: Openshift;
    clusterRoleRules: ClusterRoleRules;
    extraContainers: any[];
    initContainers: any[];
    pdb: Pdb;
}

export interface ControllerArgs {
    statusProcessors: string;
    operationProcessors: string;
    appResyncPeriod: string;
    selfHealTimeout: string;
    repoServerTimeoutSeconds: string;
}

export interface Openshift {
    enabled: boolean;
}

export interface ClusterRoleRules {
    enabled: boolean;
    rules: any[];
}

export interface InitImageClass {
    repository: string;
    tag: string;
    imagePullPolicy: PullPolicy;
}

export interface NessProbe {
    failureThreshold: number;
    initialDelaySeconds: number;
    periodSeconds: number;
    successThreshold: number;
    timeoutSeconds: number;
    enabled?: boolean;
}

export interface ControllerMetrics {
    enabled: boolean;
    applicationLabels: ApplicationLabels;
    service: PurpleService;
    serviceMonitor: PurpleServiceMonitor;
    rules: Rules;
}

export interface ApplicationLabels {
    enabled: boolean;
    labels: any[];
}

export interface Rules {
    enabled: boolean;
    spec: any[];
}

export interface Pdb {
    labels: Affinity;
    annotations: Affinity;
    enabled: boolean;
}

export interface Dex {
    enabled: boolean;
    name: string;
    extraArgs: any[];
    metrics: DexMetrics;
    image: InitImageClass;
    initImage: InitImageClass;
    env: any[];
    envFrom: any[];
    podAnnotations: Affinity;
    podLabels: Affinity;
    livenessProbe: NessProbe;
    readinessProbe: NessProbe;
    serviceAccount: ServiceAccount;
    volumeMounts: VolumeMount[];
    volumes: Volume[];
    extraVolumes: any[];
    extraVolumeMounts: any[];
    containerPortHttp: number;
    servicePortHttp: number;
    servicePortHttpName: string;
    containerPortGrpc: number;
    servicePortGrpc: number;
    servicePortGrpcName: string;
    containerPortMetrics: number;
    servicePortMetrics: number;
    nodeSelector: Affinity;
    tolerations: any[];
    affinity: Affinity;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    containerSecurityContext: Affinity;
    resources: Affinity;
    extraContainers: any[];
    initContainers: any[];
    pdb: Pdb;
}

export interface DexMetrics {
    enabled: boolean;
    service: RedisService;
    serviceMonitor: PurpleServiceMonitor;
}

export interface RedisService {
    annotations: Affinity;
    labels: Affinity;
}

export interface VolumeMount {
    name: string;
    mountPath: string;
}

export interface Volume {
    name: string;
    emptyDir: Affinity;
}

export interface ExternalRedis {
    host: string;
    password: string;
    port: number;
    existingSecret: string;
}

export interface Global {
    image: InitImageClass;
    podAnnotations: Affinity;
    podLabels: Affinity;
    securityContext: Affinity;
    imagePullSecrets: any[];
    hostAliases: any[];
    additionalLabels: Affinity;
    networkPolicy: NetworkPolicy;
}

export interface NetworkPolicy {
    create: boolean;
    defaultDenyIngress: boolean;
}

export interface Notifications {
    enabled: boolean;
    name: string;
    affinity: Affinity;
    argocdUrl: null;
    image: InitImageClass;
    imagePullSecrets: any[];
    nodeSelector: Affinity;
    updateStrategy: UpdateStrategy;
    context: Affinity;
    secret: ServiceAccount;
    logLevel: string;
    extraArgs: any[];
    extraEnv: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
    metrics: NotificationsMetrics;
    notifiers: Affinity;
    podAnnotations: Affinity;
    podLabels: Affinity;
    securityContext: SlackSecurityContext;
    containerSecurityContext: Affinity;
    resources: Affinity;
    serviceAccount: ServiceAccount;
    cm: CM;
    subscriptions: any[];
    templates: Affinity;
    tolerations: any[];
    triggers: Affinity;
    bots: Bots;
}

export interface Bots {
    slack: Slack;
}

export interface Slack {
    enabled: boolean;
    updateStrategy: UpdateStrategy;
    image: InitImageClass;
    imagePullSecrets: any[];
    service: SlackService;
    serviceAccount: ServiceAccount;
    securityContext: SlackSecurityContext;
    containerSecurityContext: Affinity;
    resources: Affinity;
    affinity: Affinity;
    tolerations: any[];
    nodeSelector: Affinity;
}

export interface SlackSecurityContext {
    runAsNonRoot: boolean;
}

export interface SlackService {
    annotations: Affinity;
    port: number;
    type: string;
}

export interface UpdateStrategy {
    type: string;
}

export interface CM {
    create: boolean;
    name: string;
}

export interface NotificationsMetrics {
    enabled: boolean;
    port: number;
    service: RedisService;
    serviceMonitor: FluffyServiceMonitor;
}

export interface FluffyServiceMonitor {
    enabled: boolean;
    selector: Affinity;
    additionalLabels: Affinity;
}

export interface Welcome3Redis {
    enabled: boolean;
    name: string;
    image: InitImageClass;
    extraArgs: any[];
    containerPort: number;
    servicePort: number;
    env: any[];
    envFrom: any[];
    podAnnotations: Affinity;
    podLabels: Affinity;
    nodeSelector: Affinity;
    tolerations: any[];
    affinity: Affinity;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    containerSecurityContext: Affinity;
    securityContext: RedisSecurityContext;
    serviceAccount: ServiceAccount;
    resources: Affinity;
    volumeMounts: any[];
    volumes: any[];
    extraContainers: any[];
    initContainers: any[];
    service: RedisService;
    metrics: RedisMetrics;
    pdb: Pdb;
}

export interface RedisMetrics {
    enabled: boolean;
    image: InitImageClass;
    containerPort: number;
    resources: Affinity;
    service: FluffyService;
    serviceMonitor: PurpleServiceMonitor;
}

export interface FluffyService {
    type: string;
    clusterIP: string;
    annotations: Affinity;
    labels: Affinity;
    servicePort: number;
    portName: string;
}

export interface RedisSecurityContext {
    runAsNonRoot: boolean;
    runAsUser: number;
}

export interface RedisHa {
    enabled: boolean;
    exporter: Openshift;
    persistentVolume: Openshift;
    redis: RedisHaRedis;
    haproxy: Haproxy;
    image: RedisHaImage;
}

export interface Haproxy {
    enabled: boolean;
    metrics: Openshift;
}

export interface RedisHaImage {
    tag: string;
}

export interface RedisHaRedis {
    masterGroupName: string;
    config: RedisConfig;
}

export interface RedisConfig {
    save: string;
}

export interface RepoServer {
    name: string;
    replicas: number;
    autoscaling: Autoscaling;
    image: InitImageClass;
    extraArgs: any[];
    env: any[];
    envFrom: any[];
    logFormat: string;
    logLevel: string;
    podAnnotations: Affinity;
    podLabels: Affinity;
    containerPort: number;
    readinessProbe: NessProbe;
    livenessProbe: NessProbe;
    volumeMounts: any[];
    volumes: any[];
    nodeSelector: Affinity;
    tolerations: any[];
    affinity: Affinity;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    containerSecurityContext: Affinity;
    resources: Affinity;
    service: ApplicationSetService;
    metrics: ApplicationSetMetrics;
    clusterAdminAccess: Openshift;
    clusterRoleRules: ClusterRoleRules;
    serviceAccount: ServiceAccount;
    extraContainers: any[];
    rbac: any[];
    copyutil: Copyutil;
    initContainers: any[];
    pdb: Pdb;
}

export interface Autoscaling {
    enabled: boolean;
    minReplicas: number;
    maxReplicas: number;
    targetCPUUtilizationPercentage: number;
    targetMemoryUtilizationPercentage: number;
}

export interface Copyutil {
    resources: Affinity;
}

export interface Server {
    name: string;
    replicas: number;
    autoscaling: Autoscaling;
    image: InitImageClass;
    extraArgs: any[];
    staticAssets: Openshift;
    env: any[];
    envFrom: any[];
    lifecycle: Affinity;
    logFormat: string;
    logLevel: string;
    podAnnotations: Affinity;
    podLabels: Affinity;
    containerPort: number;
    readinessProbe: NessProbe;
    livenessProbe: NessProbe;
    volumeMounts: any[];
    volumes: any[];
    nodeSelector: Affinity;
    tolerations: any[];
    affinity: Affinity;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    containerSecurityContext: Affinity;
    resources: Affinity;
    certificate: Certificate;
    service: ServerService;
    metrics: ApplicationSetMetrics;
    serviceAccount: ServiceAccount;
    ingress: Ingress;
    ingressGrpc: Ingress;
    route: Route;
    configEnabled: boolean;
    config: ServerConfig;
    configAnnotations: Affinity;
    rbacConfig: Affinity;
    rbacConfigAnnotations: Affinity;
    rbacConfigCreate: boolean;
    additionalApplications: any[];
    additionalProjects: any[];
    clusterAdminAccess: Openshift;
    GKEbackendConfig: GkEendConfig;
    GKEmanagedCertificate: GKEmanagedCertificate;
    GKEfrontendConfig: GkEendConfig;
    extraContainers: any[];
    initContainers: any[];
    extensions: Extensions;
    pdb: Pdb;
}

export interface GkEendConfig {
    enabled: boolean;
    spec: Affinity;
}

export interface GKEmanagedCertificate {
    enabled: boolean;
    domains: string[];
}

export interface Certificate {
    enabled: boolean;
    domain: string;
    duration: string;
    renewBefore: string;
    issuer: Issuer;
    additionalHosts: any[];
    secretName: string;
}

export interface Issuer {
    kind: string;
    name: string;
}

export interface ServerConfig {
    url: string;
    "application.instanceLabelKey": string;
}

export interface Extensions {
    enabled: boolean;
    image: InitImageClass;
    resources: Affinity;
    contents: any[];
}

export interface Route {
    enabled: boolean;
    annotations: Affinity;
    hostname: string;
    termination_type: string;
    termination_policy: string;
}

export interface ServerService {
    annotations: Affinity;
    labels: Affinity;
    type: string;
    nodePortHttp: number;
    nodePortHttps: number;
    servicePortHttp: number;
    servicePortHttps: number;
    servicePortHttpName: string;
    servicePortHttpsName: string;
    namedTargetPort: boolean;
    loadBalancerIP: string;
    loadBalancerSourceRanges: any[];
    externalIPs: any[];
    externalTrafficPolicy: string;
    sessionAffinity: string;
}

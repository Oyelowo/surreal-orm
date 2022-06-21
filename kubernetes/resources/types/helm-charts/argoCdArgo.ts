export interface IArgocdargo {
    nameOverride: string;
    fullnameOverride: string;
    kubeVersionOverride: string;
    global: Global;
    apiVersionOverrides: ApiVersionOverrides;
    createAggregateRoles: boolean;
    extraObjects: any[];
    controller: Controller;
    dex: Dex;
    redis: Redis;
    'redis-ha': Redisha;
    externalRedis: ExternalRedis;
    server: Server;
    repoServer: RepoServer;
    configs: Configs;
    openshift: ClusterAdminAccess;
    applicationSet: ApplicationSet;
    notifications: Notifications;
}
interface Notifications {
    enabled: boolean;
    name: string;
    affinity: PodAnnotations;
    argocdUrl?: any;
    image: Image;
    imagePullSecrets: any[];
    nodeSelector: PodAnnotations;
    updateStrategy: UpdateStrategy;
    context: PodAnnotations;
    secret: Secret2;
    logLevel: string;
    extraArgs: any[];
    extraEnv: any[];
    extraVolumeMounts: any[];
    extraVolumes: any[];
    metrics: Metrics5;
    notifiers: PodAnnotations;
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    securityContext: SecurityContext2;
    containerSecurityContext: PodAnnotations;
    resources: PodAnnotations;
    serviceAccount: ServiceAccount2;
    cm: Cm;
    subscriptions: any[];
    templates: PodAnnotations;
    tolerations: any[];
    triggers: PodAnnotations;
    bots: Bots;
}
interface Bots {
    slack: Slack;
}
interface Slack {
    enabled: boolean;
    updateStrategy: UpdateStrategy;
    image: Image;
    imagePullSecrets: any[];
    service: Service6;
    serviceAccount: ServiceAccount2;
    securityContext: SecurityContext2;
    containerSecurityContext: PodAnnotations;
    resources: PodAnnotations;
    affinity: PodAnnotations;
    tolerations: any[];
    nodeSelector: PodAnnotations;
}
interface Service6 {
    annotations: PodAnnotations;
    port: number;
    type: string;
}
interface Cm {
    create: boolean;
    name: string;
}
interface SecurityContext2 {
    runAsNonRoot: boolean;
}
interface Metrics5 {
    enabled: boolean;
    port: number;
    service: Service3;
    serviceMonitor: ServiceMonitor2;
}
interface ServiceMonitor2 {
    enabled: boolean;
    selector: PodAnnotations;
    additionalLabels: PodAnnotations;
}
interface Secret2 {
    create: boolean;
    annotations: PodAnnotations;
    name: string;
    items: PodAnnotations;
}
interface UpdateStrategy {
    type: string;
}
interface ApplicationSet {
    enabled: boolean;
    name: string;
    replicaCount: number;
    image: Image3;
    args: Args2;
    extraContainers: any[];
    metrics: Metrics4;
    imagePullSecrets: any[];
    service: Service;
    serviceAccount: ServiceAccount2;
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    podSecurityContext: PodAnnotations;
    securityContext: PodAnnotations;
    resources: PodAnnotations;
    nodeSelector: PodAnnotations;
    tolerations: any[];
    affinity: PodAnnotations;
    priorityClassName: string;
    extraVolumeMounts: any[];
    extraVolumes: any[];
    extraArgs: any[];
    extraEnv: any[];
    extraEnvFrom: any[];
    webhook: Webhook;
}
interface Webhook {
    ingress: Ingress2;
}
interface Ingress2 {
    enabled: boolean;
    annotations: PodAnnotations;
    labels: PodAnnotations;
    ingressClassName: string;
    hosts: any[];
    paths: string[];
    pathType: string;
    extraPaths: any[];
    tls: any[];
}
interface ServiceAccount2 {
    create: boolean;
    annotations: PodAnnotations;
    name: string;
}
interface Args2 {
    metricsAddr: string;
    probeBindAddr: string;
    enableLeaderElection: boolean;
    policy: string;
    debug: boolean;
    dryRun: boolean;
}
interface Image3 {
    repository: string;
    pullPolicy: string;
    tag: string;
}
interface Configs {
    clusterCredentials: any[];
    gpgKeysAnnotations: PodAnnotations;
    gpgKeys: PodAnnotations;
    knownHostsAnnotations: PodAnnotations;
    knownHosts: KnownHosts;
    tlsCertsAnnotations: PodAnnotations;
    tlsCerts: PodAnnotations;
    repositoryCredentials: PodAnnotations;
    credentialTemplates: PodAnnotations;
    repositories: PodAnnotations;
    secret: Secret;
    styles: string;
}
interface Secret {
    createSecret: boolean;
    annotations: PodAnnotations;
    githubSecret: string;
    gitlabSecret: string;
    bitbucketServerSecret: string;
    bitbucketUUID: string;
    gogsSecret: string;
    extra: PodAnnotations;
    argocdServerTlsConfig: PodAnnotations;
    argocdServerAdminPassword: string;
    argocdServerAdminPasswordMtime: string;
}
interface KnownHosts {
    data: Data;
}
interface Data {
    ssh_known_hosts: string;
}
interface RepoServer {
    name: string;
    replicas: number;
    autoscaling: Autoscaling;
    image: Image;
    extraArgs: any[];
    env: any[];
    envFrom: any[];
    logFormat: string;
    logLevel: string;
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    containerPort: number;
    readinessProbe: ReadinessProbe;
    livenessProbe: ReadinessProbe;
    volumeMounts: any[];
    volumes: any[];
    nodeSelector: PodAnnotations;
    tolerations: any[];
    affinity: PodAnnotations;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    containerSecurityContext: PodAnnotations;
    resources: PodAnnotations;
    service: Service;
    metrics: Metrics4;
    clusterAdminAccess: ClusterAdminAccess;
    clusterRoleRules: ClusterRoleRules;
    serviceAccount: ServiceAccount;
    extraContainers: any[];
    rbac: any[];
    copyutil: Copyutil;
    initContainers: any[];
    pdb: Pdb;
}
interface Copyutil {
    resources: PodAnnotations;
}
interface Server {
    name: string;
    replicas: number;
    autoscaling: Autoscaling;
    image: Image;
    extraArgs: any[];
    staticAssets: ClusterAdminAccess;
    env: any[];
    envFrom: any[];
    lifecycle: PodAnnotations;
    logFormat: string;
    logLevel: string;
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    containerPort: number;
    readinessProbe: ReadinessProbe;
    livenessProbe: ReadinessProbe;
    volumeMounts: any[];
    volumes: any[];
    nodeSelector: PodAnnotations;
    tolerations: any[];
    affinity: PodAnnotations;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    containerSecurityContext: PodAnnotations;
    resources: PodAnnotations;
    certificate: Certificate;
    service: Service5;
    metrics: Metrics4;
    serviceAccount: ServiceAccount;
    ingress: Ingress;
    ingressGrpc: IngressGrpc;
    route: Route;
    configEnabled: boolean;
    config: Config2;
    configAnnotations: PodAnnotations;
    rbacConfig: PodAnnotations;
    rbacConfigAnnotations: PodAnnotations;
    rbacConfigCreate: boolean;
    additionalApplications: any[];
    additionalProjects: any[];
    clusterAdminAccess: ClusterAdminAccess;
    GKEbackendConfig: GKEbackendConfig;
    GKEmanagedCertificate: GKEmanagedCertificate;
    GKEfrontendConfig: GKEbackendConfig;
    extraContainers: any[];
    initContainers: any[];
    extensions: Extensions;
    pdb: Pdb;
}
interface Extensions {
    enabled: boolean;
    image: Image;
    resources: PodAnnotations;
    contents: any[];
}
interface GKEmanagedCertificate {
    enabled: boolean;
    domains: string[];
}
interface GKEbackendConfig {
    enabled: boolean;
    spec: PodAnnotations;
}
interface Config2 {
    url: string;
    'application.instanceLabelKey': string;
}
interface Route {
    enabled: boolean;
    annotations: PodAnnotations;
    hostname: string;
    termination_type: string;
    termination_policy: string;
}
interface IngressGrpc {
    enabled: boolean;
    isAWSALB: boolean;
    annotations: PodAnnotations;
    labels: PodAnnotations;
    ingressClassName: string;
    awsALB: AwsALB;
    hosts: any[];
    paths: string[];
    pathType: string;
    extraPaths: any[];
    tls: any[];
    https: boolean;
}
interface AwsALB {
    serviceType: string;
    backendProtocolVersion: string;
}
interface Ingress {
    enabled: boolean;
    annotations: PodAnnotations;
    labels: PodAnnotations;
    ingressClassName: string;
    hosts: any[];
    paths: string[];
    pathType: string;
    extraPaths: any[];
    tls: any[];
    https: boolean;
}
interface Metrics4 {
    enabled: boolean;
    service: Service2;
    serviceMonitor: ServiceMonitor;
}
interface Service5 {
    annotations: PodAnnotations;
    labels: PodAnnotations;
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
interface Certificate {
    enabled: boolean;
    domain: string;
    duration: string;
    renewBefore: string;
    issuer: Issuer;
    additionalHosts: any[];
    secretName: string;
}
interface Issuer {
    kind: string;
    name: string;
}
interface Autoscaling {
    enabled: boolean;
    minReplicas: number;
    maxReplicas: number;
    targetCPUUtilizationPercentage: number;
    targetMemoryUtilizationPercentage: number;
}
interface ExternalRedis {
    host: string;
    password: string;
    port: number;
    existingSecret: string;
}
interface Redisha {
    enabled: boolean;
    exporter: ClusterAdminAccess;
    persistentVolume: ClusterAdminAccess;
    redis: Redis2;
    haproxy: Haproxy;
    image: Image2;
}
interface Image2 {
    tag: string;
}
interface Haproxy {
    enabled: boolean;
    metrics: ClusterAdminAccess;
}
interface Redis2 {
    masterGroupName: string;
    config: Config;
}
interface Config {
    save: string;
}
interface Redis {
    enabled: boolean;
    name: string;
    image: Image;
    extraArgs: any[];
    containerPort: number;
    servicePort: number;
    env: any[];
    envFrom: any[];
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    nodeSelector: PodAnnotations;
    tolerations: any[];
    affinity: PodAnnotations;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    containerSecurityContext: PodAnnotations;
    securityContext: SecurityContext;
    serviceAccount: ServiceAccount;
    resources: PodAnnotations;
    volumeMounts: any[];
    volumes: any[];
    extraContainers: any[];
    initContainers: any[];
    service: Service3;
    metrics: Metrics3;
    pdb: Pdb;
}
interface Metrics3 {
    enabled: boolean;
    image: Image;
    containerPort: number;
    resources: PodAnnotations;
    service: Service4;
    serviceMonitor: ServiceMonitor;
}
interface Service4 {
    type: string;
    clusterIP: string;
    annotations: PodAnnotations;
    labels: PodAnnotations;
    servicePort: number;
    portName: string;
}
interface SecurityContext {
    runAsNonRoot: boolean;
    runAsUser: number;
}
interface Dex {
    enabled: boolean;
    name: string;
    extraArgs: any[];
    metrics: Metrics2;
    image: Image;
    initImage: Image;
    env: any[];
    envFrom: any[];
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    livenessProbe: LivenessProbe;
    readinessProbe: LivenessProbe;
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
    nodeSelector: PodAnnotations;
    tolerations: any[];
    affinity: PodAnnotations;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    containerSecurityContext: PodAnnotations;
    resources: PodAnnotations;
    extraContainers: any[];
    initContainers: any[];
    pdb: Pdb;
}
interface Volume {
    name: string;
    emptyDir: PodAnnotations;
}
interface VolumeMount {
    name: string;
    mountPath: string;
}
interface LivenessProbe {
    enabled: boolean;
    failureThreshold: number;
    initialDelaySeconds: number;
    periodSeconds: number;
    successThreshold: number;
    timeoutSeconds: number;
}
interface Metrics2 {
    enabled: boolean;
    service: Service3;
    serviceMonitor: ServiceMonitor;
}
interface Service3 {
    annotations: PodAnnotations;
    labels: PodAnnotations;
}
interface Controller {
    name: string;
    image: Image;
    replicas: number;
    enableStatefulSet: boolean;
    args: Args;
    logFormat: string;
    logLevel: string;
    extraArgs: any[];
    env: any[];
    envFrom: any[];
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    containerSecurityContext: PodAnnotations;
    containerPort: number;
    readinessProbe: ReadinessProbe;
    livenessProbe: ReadinessProbe;
    volumeMounts: any[];
    volumes: any[];
    service: Service;
    nodeSelector: PodAnnotations;
    tolerations: any[];
    affinity: PodAnnotations;
    topologySpreadConstraints: any[];
    priorityClassName: string;
    resources: PodAnnotations;
    serviceAccount: ServiceAccount;
    metrics: Metrics;
    clusterAdminAccess: ClusterAdminAccess;
    clusterRoleRules: ClusterRoleRules;
    extraContainers: any[];
    initContainers: any[];
    pdb: Pdb;
}
interface Pdb {
    labels: PodAnnotations;
    annotations: PodAnnotations;
    enabled: boolean;
}
interface ClusterRoleRules {
    enabled: boolean;
    rules: any[];
}
interface ClusterAdminAccess {
    enabled: boolean;
}
interface Metrics {
    enabled: boolean;
    applicationLabels: ApplicationLabels;
    service: Service2;
    serviceMonitor: ServiceMonitor;
    rules: Rules;
}
interface Rules {
    enabled: boolean;
    spec: any[];
}
interface ServiceMonitor {
    enabled: boolean;
    interval: string;
    relabelings: any[];
    metricRelabelings: any[];
    selector: PodAnnotations;
    namespace: string;
    additionalLabels: PodAnnotations;
}
interface Service2 {
    annotations: PodAnnotations;
    labels: PodAnnotations;
    servicePort: number;
}
interface ApplicationLabels {
    enabled: boolean;
    labels: any[];
}
interface ServiceAccount {
    create: boolean;
    name: string;
    annotations: PodAnnotations;
    automountServiceAccountToken: boolean;
}
interface Service {
    annotations: PodAnnotations;
    labels: PodAnnotations;
    port: number;
    portName: string;
}
interface ReadinessProbe {
    failureThreshold: number;
    initialDelaySeconds: number;
    periodSeconds: number;
    successThreshold: number;
    timeoutSeconds: number;
}
interface Args {
    statusProcessors: string;
    operationProcessors: string;
    appResyncPeriod: string;
    selfHealTimeout: string;
    repoServerTimeoutSeconds: string;
}
interface ApiVersionOverrides {
    certmanager: string;
    ingress: string;
}
interface Global {
    image: Image;
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    securityContext: PodAnnotations;
    imagePullSecrets: any[];
    hostAliases: any[];
    additionalLabels: PodAnnotations;
    networkPolicy: NetworkPolicy;
}
interface NetworkPolicy {
    create: boolean;
    defaultDenyIngress: boolean;
}
interface PodAnnotations {}
interface Image {
    repository: string;
    tag: string;
    imagePullPolicy: string;
}

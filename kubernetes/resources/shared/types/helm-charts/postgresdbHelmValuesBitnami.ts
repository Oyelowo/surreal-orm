export interface postgresdbHelmValuesBitnami {
  global: Global;
  kubeVersion: string;
  nameOverride: string;
  fullnameOverride: string;
  clusterDomain: string;
  extraDeploy: any[];
  commonLabels: CommonAnnotations;
  commonAnnotations: CommonAnnotations;
  diagnosticMode: DiagnosticMode;
  image: Image;
  auth: Welcome6Auth;
  architecture: string;
  replication: Replication;
  containerPorts: Welcome6ContainerPorts;
  audit: Audit;
  ldap: LDAP;
  postgresqlDataDir: string;
  postgresqlSharedPreloadLibraries: string;
  shmVolume: ShmVolume;
  tls: TLS;
  primary: Primary;
  readReplicas: ReadReplicas;
  networkPolicy: NetworkPolicy;
  volumePermissions: VolumePermissions;
  serviceAccount: ServiceAccount;
  rbac: Rbac;
  psp: Psp;
  metrics: Welcome6Metrics;
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

export interface Welcome6Auth {
  enablePostgresUser: boolean;
  postgresPassword: string;
  username: string;
  password: string;
  database: string;
  replicationUsername: string;
  replicationPassword: string;
  existingSecret: string;
  usePasswordFiles: boolean;
}

export interface CommonAnnotations {}

export interface Welcome6ContainerPorts {
  postgresql: number;
}

export interface DiagnosticMode {
  enabled: boolean;
  command: string[];
  args: string[];
}

export interface Global {
  imageRegistry: string;
  imagePullSecrets: any[];
  storageClass: string;
  postgresql: Postgresql;
}

export interface Postgresql {
  auth: PostgresqlAuth;
  service: PostgresqlService;
}

export interface PostgresqlAuth {
  postgresPassword: string;
  username: string;
  password: string;
  database: string;
  existingSecret: string;
}

export interface PostgresqlService {
  ports: NodePortsClass;
}

export interface NodePortsClass {
  postgresql: string;
}

export interface Image {
  registry: string;
  repository: string;
  tag: string;
  pullPolicy: string;
  pullSecrets: any[];
  debug?: boolean;
}

export interface LDAP {
  enabled: boolean;
  url: string;
  server: string;
  port: string;
  prefix: string;
  suffix: string;
  baseDN: string;
  bindDN: string;
  bind_password: string;
  search_attr: string;
  search_filter: string;
  scheme: string;
  tls: string;
}

export interface Welcome6Metrics {
  enabled: boolean;
  image: Image;
  customMetrics: CommonAnnotations;
  extraEnvVars: any[];
  containerSecurityContext: MetricsContainerSecurityContext;
  livenessProbe: Probe;
  readinessProbe: Probe;
  startupProbe: Probe;
  customLivenessProbe: CommonAnnotations;
  customReadinessProbe: CommonAnnotations;
  customStartupProbe: CommonAnnotations;
  containerPorts: MetricsContainerPorts;
  resources: MetricsResources;
  service: MetricsService;
  serviceMonitor: ServiceMonitor;
  prometheusRule: PrometheusRule;
}

export interface MetricsContainerPorts {
  metrics: number;
}

export interface MetricsContainerSecurityContext {
  enabled: boolean;
  runAsUser: number;
  runAsNonRoot: boolean;
}

export interface Probe {
  enabled: boolean;
  initialDelaySeconds: number;
  periodSeconds: number;
  timeoutSeconds: number;
  failureThreshold: number;
  successThreshold: number;
}

export interface PrometheusRule {
  enabled: boolean;
  namespace: string;
  labels: CommonAnnotations;
  rules: any[];
}

export interface MetricsResources {
  limits: CommonAnnotations;
  requests: CommonAnnotations;
}

export interface MetricsService {
  ports: MetricsContainerPorts;
  clusterIP: string;
  sessionAffinity: string;
  annotations: Annotations;
}

export interface Annotations {
  "prometheus.io/scrape": string;
  "prometheus.io/port": string;
}

export interface ServiceMonitor {
  enabled: boolean;
  namespace: string;
  interval: string;
  scrapeTimeout: string;
  labels: CommonAnnotations;
  selector: CommonAnnotations;
  relabelings: any[];
  metricRelabelings: any[];
  honorLabels: boolean;
  jobLabel: string;
}

export interface NetworkPolicy {
  enabled: boolean;
  metrics: PrimaryAccessOnlyFromClass;
  ingressRules: IngressRules;
  egressRules: EgressRules;
}

export interface EgressRules {
  denyConnectionsToExternal: boolean;
  customRules: CommonAnnotations;
}

export interface IngressRules {
  primaryAccessOnlyFrom: PrimaryAccessOnlyFromClass;
  readReplicasAccessOnlyFrom: PrimaryAccessOnlyFromClass;
}

export interface PrimaryAccessOnlyFromClass {
  enabled: boolean;
  namespaceSelector: CommonAnnotations;
  podSelector: CommonAnnotations;
  customRules?: CommonAnnotations;
}

export interface Primary {
  configuration: string;
  pgHbaConfiguration: string;
  existingConfigmap: string;
  extendedConfiguration: string;
  existingExtendedConfigmap: string;
  initdb: Initdb;
  standby: Standby;
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  command: any[];
  args: any[];
  livenessProbe: Probe;
  readinessProbe: Probe;
  startupProbe: Probe;
  customLivenessProbe: CommonAnnotations;
  customReadinessProbe: CommonAnnotations;
  customStartupProbe: CommonAnnotations;
  lifecycleHooks: CommonAnnotations;
  resources: PrimaryResources;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: PrimaryContainerSecurityContext;
  hostAliases: any[];
  labels: CommonAnnotations;
  annotations: CommonAnnotations;
  podLabels: CommonAnnotations;
  podAnnotations: CommonAnnotations;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonAnnotations;
  nodeSelector: CommonAnnotations;
  tolerations: any[];
  topologySpreadConstraints: CommonAnnotations;
  priorityClassName: string;
  schedulerName: string;
  terminationGracePeriodSeconds: string;
  updateStrategy: UpdateStrategy;
  extraVolumeMounts: any[];
  extraVolumes: any[];
  sidecars: any[];
  initContainers: any[];
  extraPodSpec: CommonAnnotations;
  service: PrimaryService;
  persistence: Persistence;
}

export interface PrimaryContainerSecurityContext {
  enabled: boolean;
  runAsUser: number;
}

export interface Initdb {
  args: string;
  postgresqlWalDir: string;
  scripts: CommonAnnotations;
  scriptsConfigMap: string;
  scriptsSecret: string;
  user: string;
  password: string;
}

export interface NodeAffinityPreset {
  type: string;
  key: string;
  values: any[];
}

export interface Persistence {
  enabled: boolean;
  existingClaim?: string;
  mountPath: string;
  subPath: string;
  storageClass: string;
  accessModes: string[];
  size: string;
  annotations: CommonAnnotations;
  selector: CommonAnnotations;
  dataSource: CommonAnnotations;
}

export interface PodSecurityContext {
  enabled: boolean;
  fsGroup: number;
}

export interface PrimaryResources {
  limits: CommonAnnotations;
  requests: Requests;
}

export interface Requests {
  memory: string;
  cpu: string;
}

export interface PrimaryService {
  type: string;
  ports: Welcome6ContainerPorts;
  nodePorts: NodePortsClass;
  clusterIP: string;
  annotations: CommonAnnotations;
  loadBalancerIP: string;
  externalTrafficPolicy: string;
  loadBalancerSourceRanges: any[];
  extraPorts: any[];
}

export interface Standby {
  enabled: boolean;
  primaryHost: string;
  primaryPort: string;
}

export interface UpdateStrategy {
  type: string;
  rollingUpdate: CommonAnnotations;
}

export interface Psp {
  create: boolean;
}

export interface Rbac {
  create: boolean;
  rules: any[];
}

export interface ReadReplicas {
  replicaCount: number;
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  command: any[];
  args: any[];
  livenessProbe: Probe;
  readinessProbe: Probe;
  startupProbe: Probe;
  customLivenessProbe: CommonAnnotations;
  customReadinessProbe: CommonAnnotations;
  customStartupProbe: CommonAnnotations;
  lifecycleHooks: CommonAnnotations;
  resources: PrimaryResources;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: PrimaryContainerSecurityContext;
  hostAliases: any[];
  labels: CommonAnnotations;
  annotations: CommonAnnotations;
  podLabels: CommonAnnotations;
  podAnnotations: CommonAnnotations;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonAnnotations;
  nodeSelector: CommonAnnotations;
  tolerations: any[];
  topologySpreadConstraints: CommonAnnotations;
  priorityClassName: string;
  schedulerName: string;
  terminationGracePeriodSeconds: string;
  updateStrategy: UpdateStrategy;
  extraVolumeMounts: any[];
  extraVolumes: any[];
  sidecars: any[];
  initContainers: any[];
  extraPodSpec: CommonAnnotations;
  service: PrimaryService;
  persistence: Persistence;
}

export interface Replication {
  synchronousCommit: string;
  numSynchronousReplicas: number;
  applicationName: string;
}

export interface ServiceAccount {
  create: boolean;
  name: string;
  automountServiceAccountToken: boolean;
  annotations: CommonAnnotations;
}

export interface ShmVolume {
  enabled: boolean;
  sizeLimit: string;
}

export interface TLS {
  enabled: boolean;
  autoGenerated: boolean;
  preferServerCiphers: boolean;
  certificatesSecret: string;
  certFilename: string;
  certKeyFilename: string;
  certCAFilename: string;
  crlFilename: string;
}

export interface VolumePermissions {
  enabled: boolean;
  image: Image;
  resources: MetricsResources;
  containerSecurityContext: VolumePermissionsContainerSecurityContext;
}

export interface VolumePermissionsContainerSecurityContext {
  runAsUser: number;
}

// This was generated from the values.yaml file.
// TODO: automate thsi workflow

export interface MongodbHelmValuesBitnami {
  global: Global;
  nameOverride: string;
  fullnameOverride: string;
  clusterDomain: string;
  extraDeploy: any[];
  commonLabels: Affinity;
  commonAnnotations: Affinity;
  diagnosticMode: DiagnosticMode;
  image: Image;
  schedulerName: string;
  architecture: string;
  useStatefulSet: boolean;
  auth: Auth;
  tls: TLS;
  hostAliases: any[];
  replicaSetName: string;
  replicaSetHostnames: boolean;
  enableIPv6: boolean;
  directoryPerDB: boolean;
  systemLogVerbosity: number;
  disableSystemLog: boolean;
  disableJavascript: boolean;
  enableJournal: boolean;
  configuration: string;
  replicaSetConfigurationSettings: ReplicaSetConfigurationSettings;
  existingConfigmap: string;
  initdbScripts: Affinity;
  initdbScriptsConfigMap: string;
  command: any[];
  args: any[];
  extraFlags: any[];
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  annotations: Affinity;
  labels: Affinity;
  replicaCount: number;
  strategyType: string;
  podManagementPolicy: string;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: Affinity;
  nodeSelector: Affinity;
  tolerations: any[];
  topologySpreadConstraints: any[];
  podLabels: Affinity;
  podAnnotations: Affinity;
  priorityClassName: string;
  runtimeClassName: string;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: Welcome1ContainerSecurityContext;
  resources: Resources;
  livenessProbe: Probe;
  readinessProbe: Probe;
  startupProbe: Probe;
  customLivenessProbe: Affinity;
  customReadinessProbe: Affinity;
  customStartupProbe: Affinity;
  initContainers: any[];
  sidecars: any[];
  extraVolumeMounts: any[];
  extraVolumes: any[];
  pdb: Pdb;
  service: Welcome1Service;
  externalAccess: ExternalAccess;
  persistence: Welcome1Persistence;
  serviceAccount: ServiceAccount;
  rbac: Rbac;
  podSecurityPolicy: PodSecurityPolicy;
  volumePermissions: VolumePermissions;
  arbiter: Arbiter;
  hidden: Arbiter;
  metrics: Metrics;
}

export interface Affinity {}

export interface Arbiter {
  enabled: boolean;
  configuration: string;
  hostAliases?: any[];
  existingConfigmap: string;
  command: any[];
  args: any[];
  extraFlags: any[];
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  annotations: Affinity;
  labels: Affinity;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: Affinity;
  nodeSelector: Affinity;
  tolerations: any[];
  podLabels: Affinity;
  podAnnotations: Affinity;
  priorityClassName: string;
  runtimeClassName: string;
  podSecurityContext?: PodSecurityContext;
  containerSecurityContext?: ArbiterContainerSecurityContext;
  resources: Resources;
  livenessProbe: Probe;
  readinessProbe: Probe;
  customLivenessProbe: Affinity;
  customReadinessProbe: Affinity;
  initContainers: any[];
  sidecars: any[];
  extraVolumeMounts: any[];
  extraVolumes: any[];
  pdb: Pdb;
  service?: ArbiterService;
  replicaCount?: number;
  strategyType?: string;
  podManagementPolicy?: string;
  persistence?: ArbiterPersistence;
}

export interface ArbiterContainerSecurityContext {
  enabled: boolean;
  runAsUser: number;
}

export interface Probe {
  enabled: boolean;
  initialDelaySeconds: number;
  periodSeconds: number;
  timeoutSeconds: number;
  failureThreshold: number;
  successThreshold: number;
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

export interface ArbiterPersistence {
  enabled: boolean;
  medium: string;
  storageClass: string;
  accessModes: string[];
  size: string;
  annotations: Affinity;
  mountPath: string;
  subPath: string;
  volumeClaimTemplates: PurpleVolumeClaimTemplates;
}

export interface PurpleVolumeClaimTemplates {
  selector: Affinity;
  dataSource: Affinity;
}

export interface PodSecurityContext {
  enabled: boolean;
  fsGroup: number;
  sysctls: any[];
}

export interface Resources {
  limits: Affinity;
  requests: Affinity;
}

export interface ArbiterService {
  nameOverride: string;
}

export interface Auth {
  enabled: boolean;
  rootUser: string;
  rootPassword: string;
  usernames: any[];
  passwords: any[];
  databases: any[];
  username: string;
  password: string;
  database: string;
  replicaSetKey: string;
  existingSecret: string;
}

export interface Welcome1ContainerSecurityContext {
  enabled: boolean;
  runAsUser: number;
  runAsNonRoot: boolean;
}

export interface DiagnosticMode {
  enabled: boolean;
  command: string[];
  args: string[];
}

export interface ExternalAccess {
  enabled: boolean;
  autoDiscovery: VolumePermissions;
  service: ExternalAccessService;
  hidden: Hidden;
}

export interface VolumePermissions {
  enabled: boolean;
  image: Image;
  resources: Resources;
  securityContext?: SecurityContext;
}

export interface Image {
  registry: string;
  repository: string;
  tag: string;
  pullPolicy: string;
  pullSecrets?: any[];
  debug?: boolean;
}

export interface SecurityContext {
  runAsUser: number;
}

export interface Hidden {
  enabled: boolean;
  service: ExternalAccessService;
}

export interface ExternalAccessService {
  type: string;
  port: number;
  loadBalancerIPs: any[];
  loadBalancerSourceRanges: any[];
  nodePorts: any[];
  domain: string;
  annotations: Affinity;
}

export interface Global {
  imageRegistry: string;
  imagePullSecrets: any[];
  storageClass: string;
  namespaceOverride: string;
}

export interface Metrics {
  enabled: boolean;
  image: Image;
  username: string;
  password: string;
  extraFlags: string;
  extraUri: string;
  resources: Resources;
  containerPort: number;
  service: MetricsService;
  livenessProbe: Probe;
  readinessProbe: Probe;
  serviceMonitor: ServiceMonitor;
  prometheusRule: PrometheusRule;
}

export interface PrometheusRule {
  enabled: boolean;
  additionalLabels: Affinity;
  namespace: string;
  rules: Affinity;
}

export interface MetricsService {
  annotations: Annotations;
  type: string;
  port: number;
}

export interface Annotations {
  "prometheus.io/scrape": string;
  "prometheus.io/port": string;
  "prometheus.io/path": string;
}

export interface ServiceMonitor {
  enabled: boolean;
  namespace: string;
  interval: string;
  scrapeTimeout: string;
  relabellings: any[];
  metricRelabelings: any[];
  additionalLabels: Affinity;
}

export interface Welcome1Persistence {
  enabled: boolean;
  medium: string;
  existingClaim: string;
  storageClass: string;
  accessModes: string[];
  size: string;
  annotations: Affinity;
  mountPath: string;
  subPath: string;
  volumeClaimTemplates: FluffyVolumeClaimTemplates;
}

export interface FluffyVolumeClaimTemplates {
  selector: Affinity;
  requests: Affinity;
  dataSource: Affinity;
}

export interface PodSecurityPolicy {
  create: boolean;
  allowPrivilegeEscalation: boolean;
  privileged: boolean;
  spec: Affinity;
}

export interface Rbac {
  create: boolean;
  role: Role;
}

export interface Role {
  rules: any[];
}

export interface ReplicaSetConfigurationSettings {
  enabled: boolean;
  configuration: Affinity;
}

export interface Welcome1Service {
  nameOverride: string;
  type: string;
  port: number;
  portName: string;
  nodePort: string;
  clusterIP: string;
  externalIPs: any[];
  loadBalancerIP: string;
  loadBalancerSourceRanges: any[];
  annotations: Affinity;
}

export interface ServiceAccount {
  create: boolean;
  name: string;
  annotations: Affinity;
}

export interface TLS {
  enabled: boolean;
  autoGenerated: boolean;
  existingSecret: string;
  caCert: string;
  caKey: string;
  image: Image;
  extraDnsNames: any[];
  mode: string;
}

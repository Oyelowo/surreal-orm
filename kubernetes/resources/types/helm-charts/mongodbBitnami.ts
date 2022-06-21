export interface IMongodbbitnami {
  global: Global;
  nameOverride: string;
  fullnameOverride: string;
  clusterDomain: string;
  extraDeploy: any[];
  commonLabels: CommonLabels;
  commonAnnotations: CommonLabels;
  diagnosticMode: DiagnosticMode;
  image: Image;
  schedulerName: string;
  architecture: string;
  useStatefulSet: boolean;
  auth: Auth;
  tls: Tls;
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
  initdbScripts: CommonLabels;
  initdbScriptsConfigMap: string;
  command: any[];
  args: any[];
  extraFlags: any[];
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  annotations: CommonLabels;
  labels: CommonLabels;
  replicaCount: number;
  strategyType: string;
  podManagementPolicy: string;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonLabels;
  nodeSelector: CommonLabels;
  tolerations: any[];
  topologySpreadConstraints: any[];
  podLabels: CommonLabels;
  podAnnotations: CommonLabels;
  priorityClassName: string;
  runtimeClassName: string;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  resources: Resources;
  livenessProbe: LivenessProbe;
  readinessProbe: LivenessProbe;
  startupProbe: LivenessProbe;
  customLivenessProbe: CommonLabels;
  customReadinessProbe: CommonLabels;
  customStartupProbe: CommonLabels;
  initContainers: any[];
  sidecars: any[];
  extraVolumeMounts: any[];
  extraVolumes: any[];
  pdb: Pdb;
  service: Service;
  externalAccess: ExternalAccess;
  persistence: Persistence;
  serviceAccount: ServiceAccount;
  rbac: Rbac;
  podSecurityPolicy: PodSecurityPolicy;
  volumePermissions: VolumePermissions;
  arbiter: Arbiter;
  hidden: Hidden2;
  metrics: Metrics;
}
interface Metrics {
  enabled: boolean;
  image: Image3;
  username: string;
  password: string;
  extraFlags: string;
  extraUri: string;
  resources: Resources;
  containerPort: number;
  service: Service4;
  livenessProbe: LivenessProbe;
  readinessProbe: LivenessProbe;
  serviceMonitor: ServiceMonitor;
  prometheusRule: PrometheusRule;
}
interface PrometheusRule {
  enabled: boolean;
  additionalLabels: CommonLabels;
  namespace: string;
  rules: any[];
}
interface ServiceMonitor {
  enabled: boolean;
  namespace: string;
  interval: string;
  scrapeTimeout: string;
  relabellings: any[];
  metricRelabelings: any[];
  additionalLabels: CommonLabels;
}
interface Service4 {
  annotations: Annotations;
  type: string;
  port: number;
}
interface Annotations {
  'prometheus.io/scrape': string;
  'prometheus.io/port': string;
  'prometheus.io/path': string;
}
interface Hidden2 {
  enabled: boolean;
  configuration: string;
  existingConfigmap: string;
  command: any[];
  args: any[];
  extraFlags: any[];
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  annotations: CommonLabels;
  labels: CommonLabels;
  replicaCount: number;
  strategyType: string;
  podManagementPolicy: string;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonLabels;
  nodeSelector: CommonLabels;
  tolerations: any[];
  podLabels: CommonLabels;
  podAnnotations: CommonLabels;
  priorityClassName: string;
  runtimeClassName: string;
  resources: Resources;
  livenessProbe: LivenessProbe;
  readinessProbe: LivenessProbe;
  customLivenessProbe: CommonLabels;
  customReadinessProbe: CommonLabels;
  initContainers: any[];
  sidecars: any[];
  extraVolumeMounts: any[];
  extraVolumes: any[];
  pdb: Pdb;
  persistence: Persistence2;
}
interface Persistence2 {
  enabled: boolean;
  medium: string;
  storageClass: string;
  accessModes: string[];
  size: string;
  annotations: CommonLabels;
  mountPath: string;
  subPath: string;
  volumeClaimTemplates: VolumeClaimTemplates2;
}
interface VolumeClaimTemplates2 {
  selector: CommonLabels;
  dataSource: CommonLabels;
}
interface Arbiter {
  enabled: boolean;
  configuration: string;
  hostAliases: any[];
  existingConfigmap: string;
  command: any[];
  args: any[];
  extraFlags: any[];
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  annotations: CommonLabels;
  labels: CommonLabels;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonLabels;
  nodeSelector: CommonLabels;
  tolerations: any[];
  podLabels: CommonLabels;
  podAnnotations: CommonLabels;
  priorityClassName: string;
  runtimeClassName: string;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: ContainerSecurityContext2;
  resources: Resources;
  livenessProbe: LivenessProbe;
  readinessProbe: LivenessProbe;
  customLivenessProbe: CommonLabels;
  customReadinessProbe: CommonLabels;
  initContainers: any[];
  sidecars: any[];
  extraVolumeMounts: any[];
  extraVolumes: any[];
  pdb: Pdb;
  service: Service3;
}
interface Service3 {
  nameOverride: string;
}
interface ContainerSecurityContext2 {
  enabled: boolean;
  runAsUser: number;
}
interface VolumePermissions {
  enabled: boolean;
  image: Image3;
  resources: Resources;
  securityContext: SecurityContext;
}
interface SecurityContext {
  runAsUser: number;
}
interface PodSecurityPolicy {
  create: boolean;
  allowPrivilegeEscalation: boolean;
  privileged: boolean;
  spec: CommonLabels;
}
interface Rbac {
  create: boolean;
  role: Role;
}
interface Role {
  rules: any[];
}
interface ServiceAccount {
  create: boolean;
  name: string;
  annotations: CommonLabels;
}
interface Persistence {
  enabled: boolean;
  medium: string;
  existingClaim: string;
  storageClass: string;
  accessModes: string[];
  size: string;
  annotations: CommonLabels;
  mountPath: string;
  subPath: string;
  volumeClaimTemplates: VolumeClaimTemplates;
}
interface VolumeClaimTemplates {
  selector: CommonLabels;
  requests: CommonLabels;
  dataSource: CommonLabels;
}
interface ExternalAccess {
  enabled: boolean;
  autoDiscovery: AutoDiscovery;
  service: Service2;
  hidden: Hidden;
}
interface Hidden {
  enabled: boolean;
  service: Service2;
}
interface Service2 {
  type: string;
  port: number;
  loadBalancerIPs: any[];
  loadBalancerSourceRanges: any[];
  nodePorts: any[];
  domain: string;
  annotations: CommonLabels;
}
interface AutoDiscovery {
  enabled: boolean;
  image: Image3;
  resources: Resources;
}
interface Image3 {
  registry: string;
  repository: string;
  tag: string;
  pullPolicy: string;
  pullSecrets: any[];
}
interface Service {
  nameOverride: string;
  type: string;
  port: number;
  portName: string;
  nodePort: string;
  clusterIP: string;
  externalIPs: any[];
  loadBalancerIP: string;
  loadBalancerSourceRanges: any[];
  annotations: CommonLabels;
}
interface Pdb {
  create: boolean;
  minAvailable: number;
  maxUnavailable: string;
}
interface LivenessProbe {
  enabled: boolean;
  initialDelaySeconds: number;
  periodSeconds: number;
  timeoutSeconds: number;
  failureThreshold: number;
  successThreshold: number;
}
interface Resources {
  limits: CommonLabels;
  requests: CommonLabels;
}
interface ContainerSecurityContext {
  enabled: boolean;
  runAsUser: number;
  runAsNonRoot: boolean;
}
interface PodSecurityContext {
  enabled: boolean;
  fsGroup: number;
  sysctls: any[];
}
interface NodeAffinityPreset {
  type: string;
  key: string;
  values: any[];
}
interface ReplicaSetConfigurationSettings {
  enabled: boolean;
  configuration: CommonLabels;
}
interface Tls {
  enabled: boolean;
  autoGenerated: boolean;
  existingSecret: string;
  caCert: string;
  caKey: string;
  image: Image2;
  extraDnsNames: any[];
  mode: string;
}
interface Image2 {
  registry: string;
  repository: string;
  tag: string;
  pullPolicy: string;
}
interface Auth {
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
interface Image {
  registry: string;
  repository: string;
  tag: string;
  pullPolicy: string;
  pullSecrets: any[];
  debug: boolean;
}
interface DiagnosticMode {
  enabled: boolean;
  command: string[];
  args: string[];
}
interface CommonLabels {
}
interface Global {
  imageRegistry: string;
  imagePullSecrets: any[];
  storageClass: string;
  namespaceOverride: string;
}

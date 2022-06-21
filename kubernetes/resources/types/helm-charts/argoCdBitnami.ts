export interface IArgocdbitnami {
  global: Global;
  kubeVersion: string;
  nameOverride: string;
  fullnameOverride: string;
  commonLabels: CommonLabels;
  commonAnnotations: CommonLabels;
  clusterDomain: string;
  extraDeploy: any[];
  image: Image;
  controller: Controller;
  server: Server;
  repoServer: RepoServer;
  dex: Dex;
  config: Config2;
  volumePermissions: VolumePermissions;
  rbac: Rbac;
  redis: Redis;
  externalRedis: ExternalRedis;
}
interface ExternalRedis {
  host: string;
  port: number;
  password: string;
  existingSecret: string;
  existingSecretPasswordKey: string;
}
interface Redis {
  image: Image2;
  enabled: boolean;
  nameOverride: string;
  service: Service5;
  auth: Auth;
  architecture: string;
}
interface Auth {
  enabled: boolean;
  existingSecret: string;
  existingSecretPasswordKey: string;
}
interface Service5 {
  port: number;
}
interface Rbac {
  create: boolean;
}
interface VolumePermissions {
  enabled: boolean;
  image: Image2;
  resources: Resources;
  containerSecurityContext: ContainerSecurityContext2;
}
interface ContainerSecurityContext2 {
  runAsUser: number;
}
interface Image2 {
  registry: string;
  repository: string;
  tag: string;
  pullPolicy: string;
  pullSecrets: any[];
}
interface Config2 {
  knownHosts: string;
  extraKnownHosts: string;
  createExtraKnownHosts: boolean;
  styles: string;
  existingStylesConfigmap: string;
  tlsCerts: CommonLabels;
  secret: Secret;
  clusterCredentials: any[];
}
interface Secret {
  create: boolean;
  annotations: CommonLabels;
  githubSecret: string;
  gitlabSecret: string;
  bitbucketServerSecret: string;
  bitbucketUUID: string;
  gogsSecret: string;
  extra: CommonLabels;
  argocdServerTlsConfig: ArgocdServerTlsConfig;
  argocdServerAdminPassword: string;
  argocdServerAdminPasswordMtime: string;
  repositoryCredentials: CommonLabels;
}
interface ArgocdServerTlsConfig {
  key: string;
  crt: string;
}
interface Dex {
  image: Image;
  enabled: boolean;
  replicaCount: number;
  startupProbe: StartupProbe;
  livenessProbe: StartupProbe;
  readinessProbe: StartupProbe;
  customStartupProbe: CommonLabels;
  customLivenessProbe: CommonLabels;
  customReadinessProbe: CommonLabels;
  resources: Resources;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  service: Service4;
  containerPorts: ContainerPorts4;
  metrics: Metrics2;
  serviceAccount: ServiceAccount;
  command: any[];
  args: any[];
  extraArgs: any[];
  hostAliases: any[];
  podLabels: CommonLabels;
  podAnnotations: CommonLabels;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonLabels;
  nodeSelector: CommonLabels;
  tolerations: any[];
  schedulerName: string;
  topologySpreadConstraints: any[];
  updateStrategy: UpdateStrategy;
  priorityClassName: string;
  lifecycleHooks: CommonLabels;
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  extraVolumes: any[];
  extraVolumeMounts: any[];
  sidecars: any[];
  initContainers: any[];
}
interface ContainerPorts4 {
  http: number;
  grpc: number;
  metrics: number;
}
interface Service4 {
  type: string;
  ports: Ports2;
  nodePorts: NodePorts2;
  clusterIP: string;
  loadBalancerIP: string;
  loadBalancerSourceRanges: any[];
  externalTrafficPolicy: string;
  annotations: CommonLabels;
  extraPorts: any[];
  sessionAffinity: string;
  sessionAffinityConfig: CommonLabels;
}
interface NodePorts2 {
  http: string;
  grpc: string;
}
interface Ports2 {
  http: number;
  grpc: number;
}
interface RepoServer {
  replicaCount: number;
  startupProbe: StartupProbe;
  livenessProbe: StartupProbe;
  readinessProbe: StartupProbe;
  customStartupProbe: CommonLabels;
  customLivenessProbe: CommonLabels;
  customReadinessProbe: CommonLabels;
  resources: Resources;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  service: Service;
  logFormat: string;
  logLevel: string;
  containerPorts: ContainerPorts3;
  metrics: Metrics2;
  autoscaling: Autoscaling;
  serviceAccount: ServiceAccount;
  command: any[];
  args: any[];
  extraArgs: any[];
  hostAliases: any[];
  podLabels: CommonLabels;
  podAnnotations: CommonLabels;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonLabels;
  nodeSelector: CommonLabels;
  tolerations: any[];
  schedulerName: string;
  topologySpreadConstraints: any[];
  updateStrategy: UpdateStrategy;
  priorityClassName: string;
  lifecycleHooks: CommonLabels;
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  extraVolumes: any[];
  extraVolumeMounts: any[];
  sidecars: any[];
  initContainers: any[];
}
interface ContainerPorts3 {
  repoServer: number;
  metrics: number;
}
interface Server {
  replicaCount: number;
  startupProbe: StartupProbe;
  livenessProbe: StartupProbe;
  readinessProbe: StartupProbe;
  customStartupProbe: CommonLabels;
  customLivenessProbe: CommonLabels;
  customReadinessProbe: CommonLabels;
  resources: Resources;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  autoscaling: Autoscaling;
  insecure: boolean;
  logFormat: string;
  logLevel: string;
  configEnabled: boolean;
  url: string;
  config: Config;
  ingress: Ingress;
  metrics: Metrics2;
  ingressGrpc: Ingress;
  containerPorts: ContainerPorts2;
  service: Service3;
  command: any[];
  args: any[];
  extraArgs: any[];
  hostAliases: any[];
  podLabels: CommonLabels;
  podAnnotations: CommonLabels;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonLabels;
  nodeSelector: CommonLabels;
  tolerations: any[];
  schedulerName: string;
  topologySpreadConstraints: any[];
  updateStrategy: UpdateStrategy;
  priorityClassName: string;
  lifecycleHooks: CommonLabels;
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  extraVolumes: any[];
  extraVolumeMounts: any[];
  sidecars: any[];
  initContainers: any[];
  serviceAccount: ServiceAccount;
}
interface Service3 {
  type: string;
  ports: Ports;
  nodePorts: NodePorts;
  clusterIP: string;
  loadBalancerIP: string;
  loadBalancerSourceRanges: any[];
  externalTrafficPolicy: string;
  annotations: CommonLabels;
  extraPorts: any[];
  sessionAffinity: string;
  sessionAffinityConfig: CommonLabels;
}
interface NodePorts {
  http: string;
  https: string;
}
interface Ports {
  http: number;
  https: number;
}
interface ContainerPorts2 {
  http: number;
  https: number;
  metrics: number;
}
interface Metrics2 {
  enabled: boolean;
  service: Service2;
  serviceMonitor: ServiceMonitor;
}
interface Ingress {
  enabled: boolean;
  selfSigned: boolean;
  pathType: string;
  apiVersion: string;
  hostname: string;
  path: string;
  annotations: CommonLabels;
  tls: boolean;
  extraHosts: any[];
  extraPaths: any[];
  extraTls: any[];
  secrets: any[];
  ingressClassName: string;
}
interface Config {
  url: string;
  'application.instanceLabelKey': string;
  'dex.config': string;
}
interface Autoscaling {
  enabled: boolean;
  minReplicas: number;
  maxReplicas: number;
  targetCPU: number;
  targetMemory: number;
}
interface Controller {
  replicaCount: number;
  startupProbe: StartupProbe;
  livenessProbe: StartupProbe;
  readinessProbe: StartupProbe;
  customStartupProbe: CommonLabels;
  customLivenessProbe: CommonLabels;
  customReadinessProbe: CommonLabels;
  resources: Resources;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  serviceAccount: ServiceAccount;
  clusterAdminAccess: boolean;
  clusterRoleRules: any[];
  logFormat: string;
  logLevel: string;
  containerPorts: ContainerPorts;
  service: Service;
  metrics: Metrics;
  command: any[];
  defaultArgs: DefaultArgs;
  args: any[];
  extraArgs: any[];
  hostAliases: any[];
  podLabels: CommonLabels;
  podAnnotations: CommonLabels;
  podAffinityPreset: string;
  podAntiAffinityPreset: string;
  nodeAffinityPreset: NodeAffinityPreset;
  affinity: CommonLabels;
  nodeSelector: CommonLabels;
  tolerations: any[];
  schedulerName: string;
  topologySpreadConstraints: any[];
  updateStrategy: UpdateStrategy;
  priorityClassName: string;
  lifecycleHooks: CommonLabels;
  extraEnvVars: any[];
  extraEnvVarsCM: string;
  extraEnvVarsSecret: string;
  extraVolumes: any[];
  extraVolumeMounts: any[];
  sidecars: any[];
  initContainers: any[];
}
interface UpdateStrategy {
  type: string;
}
interface NodeAffinityPreset {
  type: string;
  key: string;
  values: any[];
}
interface DefaultArgs {
  statusProcessors: string;
  operationProcessors: string;
  appResyncPeriod: string;
  selfHealTimeout: string;
}
interface Metrics {
  enabled: boolean;
  service: Service2;
  serviceMonitor: ServiceMonitor;
  rules: Rules;
}
interface Rules {
  enabled: boolean;
  spec: any[];
  selector: CommonLabels;
  namespace: string;
  additionalLabels: CommonLabels;
}
interface ServiceMonitor {
  enabled: boolean;
  namespace: string;
  jobLabel: string;
  interval: string;
  scrapeTimeout: string;
  relabelings: any[];
  metricRelabelings: any[];
  selector: CommonLabels;
  honorLabels: boolean;
}
interface Service2 {
  type: string;
  port: number;
  nodePort: string;
  clusterIP: string;
  loadBalancerIP: string;
  loadBalancerSourceRanges: any[];
  externalTrafficPolicy: string;
  annotations: CommonLabels;
  sessionAffinity: string;
  sessionAffinityConfig: CommonLabels;
}
interface Service {
  type: string;
  port: number;
  nodePort: string;
  clusterIP: string;
  loadBalancerIP: string;
  loadBalancerSourceRanges: any[];
  externalTrafficPolicy: string;
  annotations: CommonLabels;
  extraPorts: any[];
  sessionAffinity: string;
  sessionAffinityConfig: CommonLabels;
}
interface ContainerPorts {
  controller: number;
  metrics: number;
}
interface ServiceAccount {
  create: boolean;
  name: string;
  automountServiceAccountToken: boolean;
  annotations: CommonLabels;
}
interface ContainerSecurityContext {
  enabled: boolean;
  runAsUser: number;
  runAsNonRoot: boolean;
}
interface PodSecurityContext {
  enabled: boolean;
  fsGroup: number;
}
interface Resources {
  limits: CommonLabels;
  requests: CommonLabels;
}
interface StartupProbe {
  enabled: boolean;
  initialDelaySeconds: number;
  periodSeconds: number;
  timeoutSeconds: number;
  failureThreshold: number;
  successThreshold: number;
}
interface Image {
  registry: string;
  repository: string;
  tag: string;
  pullPolicy: string;
  pullSecrets: any[];
  debug: boolean;
}
interface CommonLabels {
}
interface Global {
  imageRegistry: string;
  imagePullSecrets: any[];
  storageClass: string;
}

export interface ICertmanagerjetspack {
  global: Global;
  installCRDs: boolean;
  replicaCount: number;
  strategy: Strategy;
  featureGates: string;
  image: Image;
  clusterResourceNamespace: string;
  serviceAccount: ServiceAccount;
  extraArgs: any[];
  extraEnv: any[];
  resources: Strategy;
  securityContext: SecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  volumes: any[];
  volumeMounts: any[];
  podLabels: Strategy;
  nodeSelector: NodeSelector;
  ingressShim: Strategy;
  prometheus: Prometheus;
  affinity: Strategy;
  tolerations: any[];
  webhook: Webhook;
  cainjector: Cainjector;
  startupapicheck: Startupapicheck;
}
interface Startupapicheck {
  enabled: boolean;
  securityContext: SecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  timeout: string;
  backoffLimit: number;
  jobAnnotations: JobAnnotations;
  extraArgs: any[];
  resources: Strategy;
  nodeSelector: Strategy;
  affinity: Strategy;
  tolerations: any[];
  podLabels: Strategy;
  image: Image;
  rbac: Rbac2;
  serviceAccount: ServiceAccount2;
}
interface ServiceAccount2 {
  create: boolean;
  annotations: JobAnnotations;
  automountServiceAccountToken: boolean;
}
interface Rbac2 {
  annotations: JobAnnotations;
}
interface JobAnnotations {
  'helm.sh/hook': string;
  'helm.sh/hook-weight': string;
  'helm.sh/hook-delete-policy': string;
}
interface Cainjector {
  enabled: boolean;
  replicaCount: number;
  strategy: Strategy;
  securityContext: SecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  extraArgs: any[];
  resources: Strategy;
  nodeSelector: NodeSelector;
  affinity: Strategy;
  tolerations: any[];
  podLabels: Strategy;
  image: Image;
  serviceAccount: ServiceAccount;
}
interface Webhook {
  replicaCount: number;
  timeoutSeconds: number;
  config?: any;
  strategy: Strategy;
  securityContext: SecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  extraArgs: any[];
  resources: Strategy;
  livenessProbe: LivenessProbe;
  readinessProbe: LivenessProbe;
  nodeSelector: NodeSelector;
  affinity: Strategy;
  tolerations: any[];
  podLabels: Strategy;
  serviceLabels: Strategy;
  image: Image;
  serviceAccount: ServiceAccount;
  securePort: number;
  hostNetwork: boolean;
  serviceType: string;
  url: Strategy;
}
interface LivenessProbe {
  failureThreshold: number;
  initialDelaySeconds: number;
  periodSeconds: number;
  successThreshold: number;
  timeoutSeconds: number;
}
interface Prometheus {
  enabled: boolean;
  servicemonitor: Servicemonitor;
}
interface Servicemonitor {
  enabled: boolean;
  prometheusInstance: string;
  targetPort: number;
  path: string;
  interval: string;
  scrapeTimeout: string;
  labels: Strategy;
  honorLabels: boolean;
}
interface NodeSelector {
  'kubernetes.io/os': string;
}
interface ContainerSecurityContext {
  allowPrivilegeEscalation: boolean;
}
interface SecurityContext {
  runAsNonRoot: boolean;
}
interface ServiceAccount {
  create: boolean;
  automountServiceAccountToken: boolean;
}
interface Image {
  repository: string;
  pullPolicy: string;
}
interface Strategy {
}
interface Global {
  imagePullSecrets: any[];
  priorityClassName: string;
  rbac: Rbac;
  podSecurityPolicy: PodSecurityPolicy;
  logLevel: number;
  leaderElection: LeaderElection;
}
interface LeaderElection {
  namespace: string;
}
interface PodSecurityPolicy {
  enabled: boolean;
  useAppArmor: boolean;
}
interface Rbac {
  create: boolean;
  aggregateClusterRoles: boolean;
}

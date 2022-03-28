export interface SealedSecretsHelmValuesBitnami {
  kubeVersion: string;
  nameOverride: string;
  fullnameOverride: string;
  namespace: string;
  extraDeploy: any[];
  image: Image;
  createController: boolean;
  secretName: string;
  updateStatus: boolean;
  keyrenewperiod: string;
  command: any[];
  args: any[];
  livenessProbe: Probe;
  readinessProbe: Probe;
  startupProbe: Probe;
  customLivenessProbe: Affinity;
  customReadinessProbe: Affinity;
  customStartupProbe: Affinity;
  resources: Resources;
  podSecurityContext: PodSecurityContext;
  containerSecurityContext: ContainerSecurityContext;
  podLabels: Affinity;
  podAnnotations: Affinity;
  priorityClassName: string;
  affinity: Affinity;
  nodeSelector: Affinity;
  tolerations: any[];
  service: Service;
  ingress: Ingress;
  networkPolicy: NetworkPolicy;
  serviceAccount: ServiceAccount;
  rbac: Rbac;
  metrics: Metrics;
}

export interface Affinity {}

export interface ContainerSecurityContext {
  enabled: boolean;
  readOnlyRootFilesystem: boolean;
  runAsNonRoot: boolean;
  runAsUser: number;
}

export interface Image {
  registry: string;
  repository: string;
  tag: string;
  pullPolicy: string;
  pullSecrets: any[];
}

export interface Ingress {
  enabled: boolean;
  pathType: string;
  apiVersion: string;
  ingressClassName: string;
  hostname: string;
  path: string;
  annotations: null;
  tls: boolean;
  selfSigned: boolean;
  extraHosts: any[];
  extraPaths: any[];
  extraTls: any[];
  secrets: any[];
}

export interface Probe {
  enabled: boolean;
  initialDelaySeconds: number;
  periodSeconds: number;
  timeoutSeconds: number;
  failureThreshold: number;
  successThreshold: number;
}

export interface Metrics {
  serviceMonitor: ServiceMonitor;
  dashboards: Dashboards;
}

export interface Dashboards {
  create: boolean;
  labels: Affinity;
  namespace: string;
}

export interface ServiceMonitor {
  enabled: boolean;
  namespace: string;
  labels: Affinity;
  annotations: Affinity;
  interval: string;
  scrapeTimeout: string;
  metricRelabelings: any[];
  relabelings: any[];
}

export interface NetworkPolicy {
  enabled: boolean;
}

export interface PodSecurityContext {
  enabled: boolean;
  fsGroup: number;
}

export interface Rbac {
  create: boolean;
  labels: Affinity;
  pspEnabled: boolean;
}

export interface Resources {
  limits: Affinity;
  requests: Affinity;
}

export interface Service {
  type: string;
  port: number;
  nodePort: string;
  annotations: Affinity;
}

export interface ServiceAccount {
  create: boolean;
  labels: Affinity;
  name: string;
}

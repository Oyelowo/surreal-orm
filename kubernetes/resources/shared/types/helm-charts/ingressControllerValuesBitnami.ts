export interface IngressControllerValuesBitnami {
    global: Global;
    nameOverride: string;
    fullnameOverride: string;
    commonLabels: AddHeaders;
    commonAnnotations: AddHeaders;
    extraDeploy: any[];
    image: Image;
    containerPorts: ContainerPorts;
    hostAliases: any[];
    config: AddHeaders;
    proxySetHeaders: AddHeaders;
    addHeaders: AddHeaders;
    defaultBackendService: string;
    electionID: string;
    reportNodeInternalIp: boolean;
    watchIngressWithoutClass: boolean;
    ingressClassResource: IngressClassResource;
    publishService: PublishService;
    scope: PodSecurityPolicy;
    configMapNamespace: string;
    tcpConfigMapNamespace: string;
    udpConfigMapNamespace: string;
    maxmindLicenseKey: string;
    dhParam: string;
    tcp: AddHeaders;
    udp: AddHeaders;
    command: any[];
    args: any[];
    extraArgs: AddHeaders;
    extraEnvVars: any[];
    extraEnvVarsCM: string;
    extraEnvVarsSecret: string;
    kind: string;
    daemonset: Daemonset;
    replicaCount: number;
    updateStrategy: AddHeaders;
    revisionHistoryLimit: number;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: Welcome6ContainerSecurityContext;
    minReadySeconds: number;
    resources: Resources;
    livenessProbe: Welcome6LivenessProbe;
    readinessProbe: Welcome6LivenessProbe;
    customLivenessProbe: AddHeaders;
    customReadinessProbe: AddHeaders;
    lifecycle: AddHeaders;
    podLabels: AddHeaders;
    podAnnotations: AddHeaders;
    priorityClassName: string;
    hostNetwork: boolean;
    dnsPolicy: string;
    terminationGracePeriodSeconds: number;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: AddHeaders;
    nodeSelector: AddHeaders;
    tolerations: any[];
    extraVolumes: any[];
    extraVolumeMounts: any[];
    initContainers: any[];
    sidecars: any[];
    customTemplate: CustomTemplate;
    topologySpreadConstraints: any[];
    podSecurityPolicy: PodSecurityPolicy;
    defaultBackend: DefaultBackend;
    service: Welcome6Service;
    serviceAccount: ServiceAccount;
    rbac: Rbac;
    pdb: Pdb;
    autoscaling: Autoscaling;
    metrics: Metrics;
}

export interface AddHeaders {}

export interface Autoscaling {
    enabled: boolean;
    minReplicas: number;
    maxReplicas: number;
    targetCPU: string;
    targetMemory: string;
}

export interface ContainerPorts {
    http: number;
    https: number;
    metrics: number;
}

export interface Welcome6ContainerSecurityContext {
    enabled: boolean;
    allowPrivilegeEscalation: boolean;
    runAsUser: number;
    capabilities: Capabilities;
}

export interface Capabilities {
    drop: string[];
    add: string[];
}

export interface CustomTemplate {
    configMapName: string;
    configMapKey: string;
}

export interface Daemonset {
    useHostPort: boolean;
    hostPorts: Ports;
}

export interface Ports {
    http: number;
    https: number;
}

export interface DefaultBackend {
    enabled: boolean;
    hostAliases: any[];
    image: Image;
    extraArgs: AddHeaders;
    containerPort: number;
    serverBlockConfig: string;
    replicaCount: number;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: DefaultBackendContainerSecurityContext;
    resources: Resources;
    livenessProbe: DefaultBackendLivenessProbe;
    readinessProbe: DefaultBackendLivenessProbe;
    podLabels: AddHeaders;
    podAnnotations: AddHeaders;
    priorityClassName: string;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: AddHeaders;
    nodeSelector: AddHeaders;
    tolerations: any[];
    service: DefaultBackendService;
    pdb: Pdb;
}

export interface DefaultBackendContainerSecurityContext {
    enabled: boolean;
    runAsUser: number;
}

export interface Image {
    registry: string;
    repository: string;
    tag: string;
    pullPolicy: string;
    pullSecrets: any[];
}

export interface DefaultBackendLivenessProbe {
    enabled: boolean;
    httpGet: PurpleHTTPGet;
    failureThreshold: number;
    initialDelaySeconds: number;
    periodSeconds: number;
    successThreshold: number;
    timeoutSeconds: number;
}

export interface PurpleHTTPGet {
    path: string;
    port: string;
    scheme: string;
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

export interface PodSecurityContext {
    enabled: boolean;
    fsGroup: number;
}

export interface Resources {
    limits: AddHeaders;
    requests: AddHeaders;
}

export interface DefaultBackendService {
    type: string;
    port: number;
}

export interface Global {
    imageRegistry: string;
    imagePullSecrets: any[];
}

export interface IngressClassResource {
    name: string;
    enabled: boolean;
    default: boolean;
    controllerClass: string;
    parameters: AddHeaders;
}

export interface Welcome6LivenessProbe {
    enabled: boolean;
    httpGet: FluffyHTTPGet;
    failureThreshold: number;
    initialDelaySeconds: number;
    periodSeconds: number;
    successThreshold: number;
    timeoutSeconds: number;
}

export interface FluffyHTTPGet {
    path: string;
    port: number;
    scheme: string;
}

export interface Metrics {
    enabled: boolean;
    service: MetricsService;
    serviceMonitor: ServiceMonitor;
    prometheusRule: PrometheusRule;
}

export interface PrometheusRule {
    enabled: boolean;
    additionalLabels: AddHeaders;
    namespace: string;
    rules: any[];
}

export interface MetricsService {
    type: string;
    port: number;
    annotations: Annotations;
}

export interface Annotations {
    'prometheus.io/scrape': string;
    'prometheus.io/port': string;
}

export interface ServiceMonitor {
    enabled: boolean;
    namespace: string;
    interval: string;
    scrapeTimeout: string;
    selector: AddHeaders;
}

export interface PodSecurityPolicy {
    enabled: boolean;
}

export interface PublishService {
    enabled: boolean;
    pathOverride: string;
}

export interface Rbac {
    create: boolean;
}

export interface Welcome6Service {
    type: string;
    ports: Ports;
    targetPorts: TargetPorts;
    nodePorts: NodePorts;
    annotations: AddHeaders;
    labels: AddHeaders;
    clusterIP: string;
    externalIPs: any[];
    loadBalancerIP: string;
    loadBalancerSourceRanges: any[];
    externalTrafficPolicy: string;
    healthCheckNodePort: number;
}

export interface NodePorts {
    http: string;
    https: string;
    tcp: AddHeaders;
    udp: AddHeaders;
}

export interface TargetPorts {
    http: string;
    https: string;
}

export interface ServiceAccount {
    create: boolean;
    name: string;
    annotations: AddHeaders;
}

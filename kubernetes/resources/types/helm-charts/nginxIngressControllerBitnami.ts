export interface INginxingresscontrollerbitnami {
    global: Global;
    nameOverride: string;
    fullnameOverride: string;
    commonLabels: CommonLabels;
    commonAnnotations: CommonLabels;
    extraDeploy: any[];
    image: Image;
    containerPorts: ContainerPorts;
    hostAliases: any[];
    config: CommonLabels;
    proxySetHeaders: CommonLabels;
    addHeaders: CommonLabels;
    defaultBackendService: string;
    electionID: string;
    reportNodeInternalIp: boolean;
    watchIngressWithoutClass: boolean;
    ingressClassResource: IngressClassResource;
    publishService: PublishService;
    scope: Scope;
    configMapNamespace: string;
    tcpConfigMapNamespace: string;
    udpConfigMapNamespace: string;
    maxmindLicenseKey: string;
    dhParam: string;
    tcp: CommonLabels;
    udp: CommonLabels;
    command: any[];
    args: any[];
    extraArgs: CommonLabels;
    extraEnvVars: any[];
    extraEnvVarsCM: string;
    extraEnvVarsSecret: string;
    kind: string;
    daemonset: Daemonset;
    replicaCount: number;
    updateStrategy: CommonLabels;
    revisionHistoryLimit: number;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: ContainerSecurityContext;
    minReadySeconds: number;
    resources: Resources;
    livenessProbe: LivenessProbe;
    readinessProbe: LivenessProbe;
    customLivenessProbe: CommonLabels;
    customReadinessProbe: CommonLabels;
    lifecycle: CommonLabels;
    podLabels: CommonLabels;
    podAnnotations: CommonLabels;
    priorityClassName: string;
    hostNetwork: boolean;
    dnsPolicy: string;
    terminationGracePeriodSeconds: number;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: CommonLabels;
    nodeSelector: CommonLabels;
    tolerations: any[];
    extraVolumes: any[];
    extraVolumeMounts: any[];
    initContainers: any[];
    sidecars: any[];
    customTemplate: CustomTemplate;
    topologySpreadConstraints: any[];
    podSecurityPolicy: Scope;
    defaultBackend: DefaultBackend;
    service: Service2;
    serviceAccount: ServiceAccount;
    rbac: Rbac;
    pdb: Pdb;
    autoscaling: Autoscaling;
    metrics: Metrics;
}
interface Metrics {
    enabled: boolean;
    service: Service3;
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
    selector: CommonLabels;
}
interface Service3 {
    type: string;
    port: number;
    annotations: Annotations;
}
interface Annotations {
    'prometheus.io/scrape': string;
    'prometheus.io/port': string;
}
interface Autoscaling {
    enabled: boolean;
    minReplicas: number;
    maxReplicas: number;
    targetCPU: string;
    targetMemory: string;
}
interface Rbac {
    create: boolean;
}
interface ServiceAccount {
    create: boolean;
    name: string;
    annotations: CommonLabels;
}
interface Service2 {
    type: string;
    ports: HostPorts;
    targetPorts: TargetPorts;
    nodePorts: NodePorts;
    annotations: CommonLabels;
    labels: CommonLabels;
    clusterIP: string;
    externalIPs: any[];
    loadBalancerIP: string;
    loadBalancerSourceRanges: any[];
    externalTrafficPolicy: string;
    healthCheckNodePort: number;
}
interface NodePorts {
    http: string;
    https: string;
    tcp: CommonLabels;
    udp: CommonLabels;
}
interface TargetPorts {
    http: string;
    https: string;
}
interface DefaultBackend {
    enabled: boolean;
    hostAliases: any[];
    image: Image;
    extraArgs: CommonLabels;
    containerPort: number;
    serverBlockConfig: string;
    replicaCount: number;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: ContainerSecurityContext2;
    resources: Resources;
    livenessProbe: LivenessProbe2;
    readinessProbe: LivenessProbe2;
    podLabels: CommonLabels;
    podAnnotations: CommonLabels;
    priorityClassName: string;
    podAffinityPreset: string;
    podAntiAffinityPreset: string;
    nodeAffinityPreset: NodeAffinityPreset;
    affinity: CommonLabels;
    nodeSelector: CommonLabels;
    tolerations: any[];
    service: Service;
    pdb: Pdb;
}
interface Pdb {
    create: boolean;
    minAvailable: number;
    maxUnavailable: string;
}
interface Service {
    type: string;
    port: number;
}
interface LivenessProbe2 {
    enabled: boolean;
    httpGet: HttpGet2;
    failureThreshold: number;
    initialDelaySeconds: number;
    periodSeconds: number;
    successThreshold: number;
    timeoutSeconds: number;
}
interface HttpGet2 {
    path: string;
    port: string;
    scheme: string;
}
interface ContainerSecurityContext2 {
    enabled: boolean;
    runAsUser: number;
}
interface CustomTemplate {
    configMapName: string;
    configMapKey: string;
}
interface NodeAffinityPreset {
    type: string;
    key: string;
    values: any[];
}
interface LivenessProbe {
    enabled: boolean;
    httpGet: HttpGet;
    failureThreshold: number;
    initialDelaySeconds: number;
    periodSeconds: number;
    successThreshold: number;
    timeoutSeconds: number;
}
interface HttpGet {
    path: string;
    port: number;
    scheme: string;
}
interface Resources {
    limits: CommonLabels;
    requests: CommonLabels;
}
interface ContainerSecurityContext {
    enabled: boolean;
    allowPrivilegeEscalation: boolean;
    runAsUser: number;
    capabilities: Capabilities;
}
interface Capabilities {
    drop: string[];
    add: string[];
}
interface PodSecurityContext {
    enabled: boolean;
    fsGroup: number;
}
interface Daemonset {
    useHostPort: boolean;
    hostPorts: HostPorts;
}
interface HostPorts {
    http: number;
    https: number;
}
interface Scope {
    enabled: boolean;
}
interface PublishService {
    enabled: boolean;
    pathOverride: string;
}
interface IngressClassResource {
    name: string;
    enabled: boolean;
    default: boolean;
    controllerClass: string;
    parameters: CommonLabels;
}
interface ContainerPorts {
    http: number;
    https: number;
    metrics: number;
}
interface Image {
    registry: string;
    repository: string;
    tag: string;
    pullPolicy: string;
    pullSecrets: any[];
}
interface CommonLabels {}
interface Global {
    imageRegistry: string;
    imagePullSecrets: any[];
}

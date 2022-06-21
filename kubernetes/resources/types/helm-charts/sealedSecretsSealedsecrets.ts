export interface ISealedsecretssealedsecrets {
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
    livenessProbe: LivenessProbe;
    readinessProbe: LivenessProbe;
    startupProbe: LivenessProbe;
    customLivenessProbe: CustomLivenessProbe;
    customReadinessProbe: CustomLivenessProbe;
    customStartupProbe: CustomLivenessProbe;
    resources: Resources;
    podSecurityContext: PodSecurityContext;
    containerSecurityContext: ContainerSecurityContext;
    podLabels: CustomLivenessProbe;
    podAnnotations: CustomLivenessProbe;
    priorityClassName: string;
    runtimeClassName: string;
    affinity: CustomLivenessProbe;
    nodeSelector: CustomLivenessProbe;
    tolerations: any[];
    service: Service;
    ingress: Ingress;
    networkPolicy: NetworkPolicy;
    serviceAccount: ServiceAccount;
    rbac: Rbac;
    metrics: Metrics;
}
interface Metrics {
    serviceMonitor: ServiceMonitor;
    dashboards: Dashboards;
}
interface Dashboards {
    create: boolean;
    labels: CustomLivenessProbe;
    namespace: string;
}
interface ServiceMonitor {
    enabled: boolean;
    namespace: string;
    labels: CustomLivenessProbe;
    annotations: CustomLivenessProbe;
    interval: string;
    scrapeTimeout: string;
    metricRelabelings: any[];
    relabelings: any[];
}
interface Rbac {
    create: boolean;
    labels: CustomLivenessProbe;
    pspEnabled: boolean;
}
interface ServiceAccount {
    create: boolean;
    labels: CustomLivenessProbe;
    name: string;
}
interface NetworkPolicy {
    enabled: boolean;
}
interface Ingress {
    enabled: boolean;
    pathType: string;
    apiVersion: string;
    ingressClassName: string;
    hostname: string;
    path: string;
    annotations?: any;
    tls: boolean;
    selfSigned: boolean;
    extraHosts: any[];
    extraPaths: any[];
    extraTls: any[];
    secrets: any[];
}
interface Service {
    type: string;
    port: number;
    nodePort: string;
    annotations: CustomLivenessProbe;
}
interface ContainerSecurityContext {
    enabled: boolean;
    readOnlyRootFilesystem: boolean;
    runAsNonRoot: boolean;
    runAsUser: number;
}
interface PodSecurityContext {
    enabled: boolean;
    fsGroup: number;
}
interface Resources {
    limits: CustomLivenessProbe;
    requests: CustomLivenessProbe;
}
interface CustomLivenessProbe {}
interface LivenessProbe {
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
}

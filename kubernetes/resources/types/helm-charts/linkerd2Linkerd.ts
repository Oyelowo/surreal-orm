export interface ILinkerd2linkerd {
    clusterDomain: string;
    clusterNetworks: string;
    imagePullPolicy: string;
    controllerLogLevel: string;
    controllerLogFormat: string;
    controlPlaneTracing: boolean;
    controlPlaneTracingNamespace: string;
    linkerdVersion: string;
    namespace: string;
    enableEndpointSlices: boolean;
    enablePprof: boolean;
    cniEnabled: boolean;
    identityTrustAnchorsPEM: string;
    identityTrustDomain: string;
    podAnnotations: PodAnnotations;
    podLabels: PodAnnotations;
    policyController: PolicyController;
    proxy: Proxy;
    proxyInit: ProxyInit;
    imagePullSecrets: any[];
    enableH2Upgrade: boolean;
    enablePSP: boolean;
    webhookFailurePolicy: string;
    controllerImage: string;
    controllerReplicas: number;
    controllerUID: number;
    debugContainer: DebugContainer;
    identity: Identity;
    disableHeartBeat: boolean;
    proxyInjector: ProxyInjector;
    profileValidator: ProxyInjector;
    policyValidator: ProxyInjector;
    installNamespace: boolean;
    nodeSelector: NodeSelector;
}
interface NodeSelector {
    'kubernetes.io/os': string;
}
interface ProxyInjector {
    externalSecret: boolean;
    namespaceSelector: NamespaceSelector;
    crtPEM: string;
    keyPEM: string;
    caBundle: string;
}
interface NamespaceSelector {
    matchExpressions: MatchExpression[];
}
interface MatchExpression {
    key: string;
    operator: string;
    values: string[];
}
interface Identity {
    externalCA: boolean;
    issuer: Issuer;
}
interface Issuer {
    scheme: string;
    clockSkewAllowance: string;
    issuanceLifetime: string;
    tls: Tls;
}
interface Tls {
    crtPEM: string;
    keyPEM: string;
}
interface DebugContainer {
    image: Image;
}
interface ProxyInit {
    ignoreInboundPorts: string;
    ignoreOutboundPorts: string;
    skipSubnets: string;
    logLevel: string;
    logFormat: string;
    image: Image;
    resources: Resources;
    closeWaitTimeoutSecs: number;
    runAsRoot: boolean;
    xtMountPath: XtMountPath;
}
interface XtMountPath {
    mountPath: string;
    name: string;
}
interface Proxy {
    enableExternalProfiles: boolean;
    outboundConnectTimeout: string;
    inboundConnectTimeout: string;
    image: Image;
    logLevel: string;
    logFormat: string;
    ports: Ports;
    cores: number;
    resources: Resources;
    uid: number;
    waitBeforeExitSeconds: number;
    await: boolean;
    requireIdentityOnInboundPorts: string;
    opaquePorts: string;
}
interface Ports {
    admin: number;
    control: number;
    inbound: number;
    outbound: number;
}
interface PolicyController {
    image: Image;
    defaultAllowPolicy: string;
    logLevel: string;
    resources: Resources;
}
interface Resources {
    cpu: Cpu;
    memory: Cpu;
}
interface Cpu {
    limit: string;
    request: string;
}
interface Image {
    name: string;
    pullPolicy: string;
    version: string;
}
interface PodAnnotations {}

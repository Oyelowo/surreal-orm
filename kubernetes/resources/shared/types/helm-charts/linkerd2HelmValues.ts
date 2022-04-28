export interface Linkerd2HelmValues {
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
    podAnnotations: Pod;
    podLabels: Pod;
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
    proxyInjector: Tor;
    profileValidator: Tor;
    policyValidator: Tor;
    installNamespace: boolean;
    nodeSelector: NodeSelector;
}

export interface DebugContainer {
    image: Image;
}

export interface Image {
    name: string;
    pullPolicy: string;
    version: string;
}

export interface Identity {
    externalCA: boolean;
    issuer: Issuer;
}

export interface Issuer {
    scheme: string;
    clockSkewAllowance: string;
    issuanceLifetime: string;
    tls: TLS;
}

export interface TLS {
    crtPEM: string;
    keyPEM: string;
}

export interface NodeSelector {
    "kubernetes.io/os": string;
}

export interface Pod {
}

export interface PolicyController {
    image: Image;
    defaultAllowPolicy: string;
    logLevel: string;
    resources: Resources;
}

export interface Resources {
    cpu: CPU;
    memory: CPU;
}

export interface CPU {
    limit: string;
    request: string;
}

export interface Tor {
    externalSecret: boolean;
    namespaceSelector: NamespaceSelector;
    crtPEM: string;
    keyPEM: string;
    caBundle: string;
}

export interface NamespaceSelector {
    matchExpressions: MatchExpression[];
}

export interface MatchExpression {
    key: string;
    operator: string;
    values: string[];
}

export interface Proxy {
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

export interface Ports {
    admin: number;
    control: number;
    inbound: number;
    outbound: number;
}

export interface ProxyInit {
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

export interface XtMountPath {
    mountPath: string;
    name: string;
}

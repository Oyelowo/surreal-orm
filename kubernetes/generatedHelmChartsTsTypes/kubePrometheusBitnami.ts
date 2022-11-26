// Don't Edit. This is autogenerated.
export interface IKubePrometheusBitnami {
	global: Global;
	kubeVersion: string;
	nameOverride: string;
	fullnameOverride: string;
	namespaceOverride: string;
	commonAnnotations: CommonAnnotations;
	commonLabels: CommonAnnotations;
	extraDeploy: any[];
	clusterDomain: string;
	operator: Operator;
	prometheus: Prometheus;
	alertmanager: Alertmanager;
	exporters: Exporters;
	"node-exporter": Nodeexporter2;
	"kube-state-metrics": Kubestatemetrics;
	kubelet: Kubelet;
	blackboxExporter: BlackboxExporter;
	kubeApiServer: KubeApiServer;
	kubeControllerManager: KubeControllerManager;
	kubeScheduler: KubeScheduler;
	coreDns: CoreDns;
	kubeProxy: KubeProxy;
	rbac: Rbac;
}
interface Rbac {
	create: boolean;
	pspEnabled: boolean;
}
interface KubeProxy {
	enabled: boolean;
	endpoints: any[];
	namespace: string;
	service: Service6;
	serviceMonitor: ServiceMonitor10;
}
interface ServiceMonitor10 {
	https: boolean;
	interval: string;
	metricRelabelings: any[];
	relabelings: any[];
	labels: CommonAnnotations;
	annotations: CommonAnnotations;
}
interface CoreDns {
	enabled: boolean;
	namespace: string;
	service: Service6;
	serviceMonitor: ServiceMonitor7;
}
interface KubeScheduler {
	enabled: boolean;
	endpoints: any[];
	namespace: string;
	service: Service6;
	serviceMonitor: ServiceMonitor9;
}
interface ServiceMonitor9 {
	interval: string;
	https: boolean;
	insecureSkipVerify: string;
	serverName: string;
	metricRelabelings: any[];
	relabelings: any[];
	labels: CommonAnnotations;
	annotations: CommonAnnotations;
}
interface KubeControllerManager {
	enabled: boolean;
	endpoints: any[];
	namespace: string;
	service: Service6;
	serviceMonitor: ServiceMonitor8;
}
interface ServiceMonitor8 {
	interval: string;
	https: boolean;
	insecureSkipVerify: string;
	serverName: string;
	metricRelabelings: any[];
	relabelings: any[];
}
interface Service6 {
	enabled: boolean;
	ports: Ports;
	targetPorts: Ports;
	selector: CommonAnnotations;
}
interface KubeApiServer {
	enabled: boolean;
	serviceMonitor: ServiceMonitor7;
}
interface ServiceMonitor7 {
	interval: string;
	metricRelabelings: any[];
	relabelings: any[];
}
interface BlackboxExporter {
	enabled: boolean;
	image: Image;
	extraEnvVars: any[];
	extraEnvVarsCM: string;
	extraEnvVarsSecret: string;
	command: any[];
	args: any[];
	replicaCount: number;
	livenessProbe: LivenessProbe;
	readinessProbe: LivenessProbe;
	startupProbe: LivenessProbe;
	customLivenessProbe: CommonAnnotations;
	customReadinessProbe: CommonAnnotations;
	customStartupProbe: CommonAnnotations;
	configuration: string;
	existingConfigMap: string;
	containerPorts: Ports;
	serviceAccount: ServiceAccount;
	resources: Resources;
	podSecurityContext: PodSecurityContext2;
	containerSecurityContext: ContainerSecurityContext2;
	lifecycleHooks: CommonAnnotations;
	hostAliases: any[];
	podLabels: CommonAnnotations;
	podAnnotations: CommonAnnotations;
	podAffinityPreset: string;
	podAntiAffinityPreset: string;
	nodeAffinityPreset: NodeAffinityPreset;
	affinity: CommonAnnotations;
	nodeSelector: CommonAnnotations;
	tolerations: any[];
	topologySpreadConstraints: any[];
	priorityClassName: string;
	schedulerName: string;
	terminationGracePeriodSeconds: string;
	updateStrategy: UpdateStrategy;
	extraVolumes: any[];
	extraVolumeMounts: any[];
	sidecars: any[];
	initContainers: any[];
	service: Service5;
}
interface Service5 {
	type: string;
	ports: Ports;
	nodePorts: NodePorts;
	sessionAffinity: string;
	sessionAffinityConfig: CommonAnnotations;
	clusterIP: string;
	loadBalancerIP: string;
	loadBalancerSourceRanges: any[];
	externalTrafficPolicy: string;
	annotations: CommonAnnotations;
	extraPorts: any[];
}
interface UpdateStrategy {
	type: string;
}
interface ContainerSecurityContext2 {
	enabled: boolean;
	runAsUser: number;
	runAsNonRoot: boolean;
}
interface PodSecurityContext2 {
	enabled: boolean;
	fsGroup: number;
}
interface Kubelet {
	enabled: boolean;
	namespace: string;
	serviceMonitor: ServiceMonitor6;
}
interface ServiceMonitor6 {
	https: boolean;
	interval: string;
	metricRelabelings: any[];
	relabelings: any[];
	cAdvisorMetricRelabelings: any[];
	cAdvisorRelabelings: any[];
	labels: CommonAnnotations;
	annotations: CommonAnnotations;
}
interface Kubestatemetrics {
	serviceMonitor: ServiceMonitor5;
}
interface ServiceMonitor5 {
	enabled: boolean;
	honorLabels: boolean;
}
interface Nodeexporter2 {
	service: Service4;
	serviceMonitor: ServiceMonitor4;
	extraArgs: ExtraArgs;
}
interface ExtraArgs {
	"collector.filesystem.ignored-mount-points": string;
	"collector.filesystem.ignored-fs-types": string;
}
interface ServiceMonitor4 {
	enabled: boolean;
	jobLabel: string;
}
interface Service4 {
	labels: Labels;
}
interface Labels {
	jobLabel: string;
}
interface Exporters {
	"node-exporter": Nodeexporter;
	"kube-state-metrics": Nodeexporter;
}
interface Nodeexporter {
	enabled: boolean;
}
interface Alertmanager {
	enabled: boolean;
	image: Image2;
	serviceAccount: ServiceAccount;
	podSecurityContext: PodSecurityContext;
	containerSecurityContext: ContainerSecurityContext;
	pdb: Pdb;
	service: Service;
	serviceMonitor: ServiceMonitor3;
	ingress: Ingress;
	externalUrl: string;
	resources: CommonAnnotations;
	podAffinityPreset: string;
	podAntiAffinityPreset: string;
	nodeAffinityPreset: NodeAffinityPreset;
	affinity: CommonAnnotations;
	nodeSelector: CommonAnnotations;
	tolerations: any[];
	config: Config;
	templateFiles: CommonAnnotations;
	externalConfig: boolean;
	replicaCount: number;
	livenessProbe: LivenessProbe2;
	readinessProbe: LivenessProbe2;
	logLevel: string;
	logFormat: string;
	podMetadata: PodMetadata;
	secrets: any[];
	configMaps: any[];
	retention: string;
	storageSpec: CommonAnnotations;
	persistence: Persistence;
	paused: boolean;
	listenLocal: boolean;
	containers: any[];
	volumes: any[];
	volumeMounts: any[];
	priorityClassName: string;
	additionalPeers: any[];
	routePrefix: string;
	portName: string;
	configNamespaceSelector: CommonAnnotations;
	configSelector: CommonAnnotations;
	configuration: CommonAnnotations;
}
interface Config {
	global: Global2;
	route: Route2;
	receivers: Receiver[];
}
interface Receiver {
	name: string;
}
interface Route2 {
	group_by: string[];
	group_wait: string;
	group_interval: string;
	repeat_interval: string;
	receiver: string;
	routes: Route[];
}
interface Route {
	match: Match;
	receiver: string;
}
interface Match {
	alertname: string;
}
interface Global2 {
	resolve_timeout: string;
}
interface ServiceMonitor3 {
	enabled: boolean;
	interval: string;
	metricRelabelings: any[];
	relabelings: any[];
	jobLabel: string;
	scrapeTimeout: string;
	selector: CommonAnnotations;
	labels: CommonAnnotations;
	annotations: CommonAnnotations;
	honorLabels: boolean;
}
interface Prometheus {
	enabled: boolean;
	image: Image2;
	serviceAccount: ServiceAccount;
	podSecurityContext: PodSecurityContext;
	containerSecurityContext: ContainerSecurityContext;
	pdb: Pdb;
	service: Service2;
	serviceMonitor: ServiceMonitor2;
	ingress: Ingress;
	externalUrl: string;
	resources: CommonAnnotations;
	podAffinityPreset: string;
	podAntiAffinityPreset: string;
	nodeAffinityPreset: NodeAffinityPreset;
	affinity: CommonAnnotations;
	nodeSelector: CommonAnnotations;
	tolerations: any[];
	scrapeInterval: string;
	evaluationInterval: string;
	listenLocal: boolean;
	livenessProbe: LivenessProbe2;
	readinessProbe: LivenessProbe2;
	startupProbe: LivenessProbe2;
	enableAdminAPI: boolean;
	enableFeatures: any[];
	alertingEndpoints: any[];
	externalLabels: CommonAnnotations;
	replicaExternalLabelName: string;
	replicaExternalLabelNameClear: boolean;
	routePrefix: string;
	prometheusExternalLabelName: string;
	prometheusExternalLabelNameClear: boolean;
	secrets: any[];
	configMaps: any[];
	querySpec: CommonAnnotations;
	ruleNamespaceSelector: CommonAnnotations;
	ruleSelector: CommonAnnotations;
	serviceMonitorSelector: CommonAnnotations;
	serviceMonitorNamespaceSelector: CommonAnnotations;
	podMonitorSelector: CommonAnnotations;
	podMonitorNamespaceSelector: CommonAnnotations;
	probeSelector: CommonAnnotations;
	probeNamespaceSelector: CommonAnnotations;
	retention: string;
	retentionSize: string;
	disableCompaction: boolean;
	walCompression: boolean;
	paused: boolean;
	replicaCount: number;
	shards: number;
	logLevel: string;
	logFormat: string;
	podMetadata: PodMetadata;
	remoteRead: any[];
	remoteWrite: any[];
	storageSpec: CommonAnnotations;
	persistence: Persistence;
	priorityClassName: string;
	containers: any[];
	initContainers: any[];
	volumes: any[];
	volumeMounts: any[];
	additionalPrometheusRules: any[];
	additionalScrapeConfigs: AdditionalScrapeConfigs;
	additionalScrapeConfigsExternal: AdditionalScrapeConfigsExternal;
	additionalAlertRelabelConfigsExternal: AdditionalScrapeConfigsExternal;
	thanos: Thanos;
	portName: string;
}
interface Thanos {
	create: boolean;
	image: Image;
	containerSecurityContext: ContainerSecurityContext;
	prometheusUrl: string;
	extraArgs: any[];
	objectStorageConfig: CommonAnnotations;
	extraVolumeMounts: any[];
	resources: Resources;
	livenessProbe: LivenessProbe2;
	readinessProbe: LivenessProbe2;
	service: Service3;
	ingress: Ingress;
}
interface Service3 {
	type: string;
	ports: Ports2;
	clusterIP: string;
	nodePorts: NodePorts2;
	loadBalancerIP: string;
	loadBalancerSourceRanges: any[];
	annotations: CommonAnnotations;
	extraPorts: any[];
	externalTrafficPolicy: string;
	sessionAffinity: string;
	sessionAffinityConfig: CommonAnnotations;
}
interface NodePorts2 {
	grpc: string;
}
interface Ports2 {
	grpc: number;
}
interface Resources {
	limits: CommonAnnotations;
	requests: CommonAnnotations;
}
interface AdditionalScrapeConfigsExternal {
	enabled: boolean;
	name: string;
	key: string;
}
interface AdditionalScrapeConfigs {
	enabled: boolean;
	type: string;
	external: External;
	internal: Internal;
}
interface Internal {
	jobList: any[];
}
interface External {
	name: string;
	key: string;
}
interface Persistence {
	enabled: boolean;
	storageClass: string;
	accessModes: string[];
	size: string;
	annotations: CommonAnnotations;
}
interface PodMetadata {
	labels: CommonAnnotations;
	annotations: CommonAnnotations;
}
interface LivenessProbe2 {
	enabled: boolean;
	path: string;
	initialDelaySeconds: number;
	failureThreshold: number;
	periodSeconds: number;
	successThreshold: number;
	timeoutSeconds: number;
}
interface Ingress {
	enabled: boolean;
	pathType: string;
	apiVersion: string;
	hostname: string;
	path: string;
	annotations: CommonAnnotations;
	ingressClassName: string;
	tls: boolean;
	selfSigned: boolean;
	extraHosts: any[];
	extraPaths: any[];
	extraTls: any[];
	secrets: any[];
	extraRules: any[];
}
interface ServiceMonitor2 {
	enabled: boolean;
	interval: string;
	metricRelabelings: any[];
	relabelings: any[];
}
interface Service2 {
	type: string;
	ports: Ports;
	clusterIP: string;
	nodePorts: NodePorts;
	loadBalancerIP: string;
	loadBalancerSourceRanges: any[];
	externalTrafficPolicy: string;
	healthCheckNodePort: string;
	annotations: CommonAnnotations;
	sessionAffinity: string;
	sessionAffinityConfig: CommonAnnotations;
}
interface Pdb {
	create: boolean;
	minAvailable: number;
	maxUnavailable: string;
}
interface Image2 {
	registry: string;
	repository: string;
	tag: string;
	digest: string;
	pullSecrets: any[];
}
interface Operator {
	enabled: boolean;
	image: Image;
	extraArgs: any[];
	command: any[];
	args: any[];
	lifecycleHooks: CommonAnnotations;
	extraEnvVars: any[];
	extraEnvVarsCM: string;
	extraEnvVarsSecret: string;
	extraVolumes: any[];
	extraVolumeMounts: any[];
	sidecars: any[];
	initContainers: any[];
	hostAliases: any[];
	serviceAccount: ServiceAccount;
	schedulerName: string;
	terminationGracePeriodSeconds: string;
	topologySpreadConstraints: any[];
	podSecurityContext: PodSecurityContext;
	containerSecurityContext: ContainerSecurityContext;
	service: Service;
	serviceMonitor: ServiceMonitor;
	resources: CommonAnnotations;
	podAffinityPreset: string;
	podAntiAffinityPreset: string;
	nodeAffinityPreset: NodeAffinityPreset;
	affinity: CommonAnnotations;
	nodeSelector: CommonAnnotations;
	tolerations: any[];
	podAnnotations: CommonAnnotations;
	podLabels: CommonAnnotations;
	priorityClassName: string;
	livenessProbe: LivenessProbe;
	readinessProbe: LivenessProbe;
	startupProbe: LivenessProbe;
	customLivenessProbe: CommonAnnotations;
	customReadinessProbe: CommonAnnotations;
	customStartupProbe: CommonAnnotations;
	logLevel: string;
	logFormat: string;
	configReloaderResources: CommonAnnotations;
	kubeletService: KubeletService;
	prometheusConfigReloader: PrometheusConfigReloader;
}
interface PrometheusConfigReloader {
	image: CommonAnnotations;
	containerSecurityContext: ContainerSecurityContext;
	livenessProbe: LivenessProbe;
	readinessProbe: LivenessProbe;
}
interface KubeletService {
	enabled: boolean;
	namespace: string;
}
interface LivenessProbe {
	enabled: boolean;
	initialDelaySeconds: number;
	periodSeconds: number;
	timeoutSeconds: number;
	failureThreshold: number;
	successThreshold: number;
}
interface NodeAffinityPreset {
	type: string;
	key: string;
	values: any[];
}
interface ServiceMonitor {
	enabled: boolean;
	interval: string;
	metricRelabelings: any[];
	relabelings: any[];
	scrapeTimeout: string;
	labels: CommonAnnotations;
	annotations: CommonAnnotations;
}
interface Service {
	type: string;
	ports: Ports;
	clusterIP: string;
	nodePorts: NodePorts;
	loadBalancerIP: string;
	loadBalancerSourceRanges: any[];
	externalTrafficPolicy: string;
	healthCheckNodePort: string;
	annotations: CommonAnnotations;
	extraPorts: any[];
	sessionAffinity: string;
	sessionAffinityConfig: CommonAnnotations;
}
interface NodePorts {
	http: string;
}
interface Ports {
	http: number;
}
interface ContainerSecurityContext {
	enabled: boolean;
	capabilities: Capabilities;
	runAsNonRoot: boolean;
	allowPrivilegeEscalation: boolean;
	readOnlyRootFilesystem: boolean;
}
interface Capabilities {
	drop: string[];
}
interface PodSecurityContext {
	enabled: boolean;
	runAsUser: number;
	fsGroup: number;
}
interface ServiceAccount {
	create: boolean;
	name: string;
	automountServiceAccountToken: boolean;
	annotations: CommonAnnotations;
}
interface Image {
	registry: string;
	repository: string;
	tag: string;
	digest: string;
	pullPolicy: string;
	pullSecrets: any[];
}
interface CommonAnnotations {}
interface Global {
	imageRegistry: string;
	imagePullSecrets: any[];
	storageClass: string;
}

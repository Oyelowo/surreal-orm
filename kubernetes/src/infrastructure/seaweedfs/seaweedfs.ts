import { ISeaweedfsOyelowo } from "../../../generatedHelmChartsTsTypes/seaweedfsOyelowo.js";
import * as k8s from "@pulumi/kubernetes";
import { namespaces } from "../../types/ownTypes.js";
import { helmChartsInfo } from "../../shared/helmChartInfo.js";
import { DeepPartial } from "../../types/ownTypes.js";
import { seaweedFsProvider } from "./settings.js";
import { seaweedFsTikvSettings } from "./tikvCluster.js";

const seaweedFsValues: DeepPartial<ISeaweedfsOyelowo> = {
	global: {
		imageName: "chrislusf/seaweedfs",
		imagePullPolicy: "IfNotPresent",
		imagePullSecrets: "imagepullsecret",
		restartPolicy: "Always",
		loggingLevel: 1,
		enableSecurity: false,
		// replicationPlacment: '',
		monitoring: {
			enabled: false,
			gatewayHost: undefined,
			gatewayPort: undefined,
		},
		// if enabled will use global.replicationPlacment and override master & filer defaultReplicaPlacement config
		enableReplication: false,
		/* 
    #  replication type is XYZ:
    # X number of replica in other data centers
    # Y number of replica in other racks in the same data center
    # Z number of replica in other servers in the same rack
        */
		replicationPlacment: "001",
		extraEnvironmentVars: {
			WEED_CLUSTER_DEFAULT: "sw",
			WEED_CLUSTER_SW_MASTER: "seaweedfs-master:9333",
			WEED_CLUSTER_SW_FILER: "seaweedfs-filer-client:8888",
		},
	},
	image: {
		registry: "",
		repository: "",
	},
	master: {
		enabled: true,
		repository: undefined,
		imageName: undefined,
		imageTag: undefined,
		imageOverride: undefined,
		restartPolicy: undefined,
		replicas: 1,
		port: 9333,
		grpcPort: 19_333,
		metricsPort: 9327,
		ipBind: "0.0.0.0",
		volumePreallocate: false,
		volumeSizeLimitMB: 1000,
		loggingOverrideLevel: undefined,
		// number of seconds between heartbeats, default 5,
		pulseSeconds: undefined,
		//  threshold to vacuum and reclaim spaces, default 0.3 (30%)
		garbageThreshold: undefined,
		// Prometheus push interval in seconds, default 15
		metricsIntervalSec: 15,
		/* 
            #  replication type is XYZ:
    # X number of replica in other data centers
    # Y number of replica in other racks in the same data center
    # Z number of replica in other servers in the same rack
        */
		defaultReplication: "000",
		// Disable http request, only gRpc operations are allowed
		disableHttp: false,
		// can use ANY storage-class , example with local-path-provisioner

		data: {
			type: "persistentVolumeClaim",
			// type: 'hostPath',
			size: "0.4Ti",
			// size: "24Ti",
			storageClass: "local-path-provisioner",
			// storageClass: ''
		},
		logs: {
			type: "hostPath",
			size: "",
			storageClass: "",
		},
		initContainers: "",
		extraVolumes: "",
		extraVolumeMounts: "",
		/* 
            # Resource requests, limits, etc. for the master cluster placement. This
    # should map directly to the value of the resources field for a PodSpec,
    # formatted as a multi-line string. By default no direct resource request
    # is made.
        */
		// resources: `
		//     requests: '',
		//     limits: ''`,
		/* 
        updatePartition is used to control a careful rolling update of SeaweedFS
         masters.
        */
		updatePartition: 0,
		/* 
            # Affinity Settings
    # Commenting out or setting as empty the affinity variable, will allow
    # deployment to single node services such as Minikube
        */
		// affinity: `
		//     /*
		//            podAntiAffinity:
		//  requiredDuringSchedulingIgnoredDuringExecution:
		//    - labelSelector:
		//        matchLabels:
		//          app: {{ template "seaweedfs.name" . }}
		//          release: "{{ .Release.Name }}"
		//          component: master
		//      topologyKey: kubernetes.io/hostname
		//     */
		// `
		// affinity: {
		//     podAntiAffinity: {
		//         requiredDuringSchedulingIgnoredDuringExecution: [
		//             {
		//                 labelSelector: {
		//                     matchLabels: {
		//                         app: `{{ template "seaweedfs.name" . }}`,
		//                         release: "{{ .Release.Name }}",
		//                         component: 'master'
		//                     },
		//                     topologyKey: 'kubernetes.io/hostname'
		//                 }
		//             }
		//         ]
		//     }
		// } as any,

		/*
            # Toleration Settings for master pods
    # This should be a multi-line string matching the Toleration array
    # in a PodSpec.
        */
		tolerations: "",
		/* 
            # nodeSelector labels for master pod assignment, formatted as a muli-line string.
    # ref: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/#nodeselector
    # Example:
        */
		// nodeSelector: 'beta.kubernetes.io/arch: amd64'
		nodeSelector: 'sw-backend: "true"',
		/* 
            # used to assign priority to master pods
    # ref: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
        */
		priorityClassName: "",
		ingress: {
			enabled: false,
			className: "nginx",
			annotations: {
				"nginx.ingress.kubernetes.io/auth-type": "basic",
				"nginx.ingress.kubernetes.io/auth-secret":
					"default/ingress-basic-auth-secret",
				"nginx.ingress.kubernetes.io/auth-realm":
					"Authentication Required - SW-Master",
				"nginx.ingress.kubernetes.io/service-upstream": "true",
				"nginx.ingress.kubernetes.io/rewrite-target": "/$1",
				"nginx.ingress.kubernetes.io/use-regex": "true",
				"nginx.ingress.kubernetes.io/enable-rewrite-log": "true",
				"nginx.ingress.kubernetes.io/ssl-redirect": "false",
				"nginx.ingress.kubernetes.io/force-ssl-redirect": "false",
				"nginx.ingress.kubernetes.io/configuration-snippet": `|
                sub_filter '<head>' '<head> <base href="/sw-master/">'; #add base url
                sub_filter '="/' '="./';                                #make absolute paths to relative
                sub_filter '=/' '=./';
                sub_filter '/seaweedfsstatic' './seaweedfsstatic';
                sub_filter_once off;`,
			},
		},
		extraEnvironmentVars: {
			WEED_MASTER_VOLUME_GROWTH_COPY_1: 7,
			WEED_MASTER_VOLUME_GROWTH_COPY_2: 6,
			WEED_MASTER_VOLUME_GROWTH_COPY_3: 3,
			WEED_MASTER_VOLUME_GROWTH_COPY_OTHER: 1,
		},
	},
	volume: {
		data: {
			type: "persistentVolumeClaim",
			size: "24Ti",
			storageClass: "local-path-provisioner",
			// type: 'hostPath',
			// size: '',
			// storageClass: '',
		},
		idx: {},
		logs: {},
	},
	filer: {
		// This is important. The base image does not have TiKV filer configured
		imageOverride: "chrislusf/seaweedfs:3.29_full",
		replicas: 1,
		port: 8888,
		grpcPort: 18_888,
		metricsPort: 9327,
		/* 
            #  replication type is XYZ:
    # X number of replica in other data centers
    # Y number of replica in other racks in the same data center
    # Z number of replica in other servers in the same rack
        */
		defaultReplicaPlacement: "000",
		//   split files larger than the limit, default 32
		maxMB: undefined,
		//  # encrypt data on volume servers
		encryptVolumeData: false,
		//     # Limit sub dir listing size (default 100000)
		dirListLimit: 1000,
		data: {
			type: "persistentVolumeClaim",
			size: "0.4Ti",
			//  size: "24Ti",
			storageClass: "local-path-provisioner",
		},
		// affinity: {
		//     podAntiAffinity: {
		//         requiredDuringSchedulingIgnoredDuringExecution: [
		//             {
		//                 labelSelector: {
		//                     matchLabels: {
		//                         app: `{{ template "seaweedfs.name" . }}`,
		//                         release: "{{ .Release.Name }}",
		//                         component: 'master'
		//                     },
		//                     topologyKey: 'kubernetes.io/hostname'
		//                 }
		//             }
		//         ]
		//     }
		// } as any,
		ingress: {
			annotations: {
				/* 
                            nginx.ingress.kubernetes.io/auth-type: 'basic'
            nginx.ingress.kubernetes.io/auth-secret: 'default/ingress-basic-auth-secret'
            nginx.ingress.kubernetes.io/auth-realm: 'Authentication Required - SW-Filer'
            nginx.ingress.kubernetes.io/service-upstream: 'true'
            nginx.ingress.kubernetes.io/rewrite-target: /$1
            nginx.ingress.kubernetes.io/use-regex: 'true'
            nginx.ingress.kubernetes.io/enable-rewrite-log: 'true'
            nginx.ingress.kubernetes.io/ssl-redirect: 'false'
            nginx.ingress.kubernetes.io/force-ssl-redirect: 'false'
            nginx.ingress.kubernetes.io/configuration-snippet: |
                sub_filter '<head>' '<head> <base href="/sw-filer/">'; #add base url
                sub_filter '="/' '="./';                               #make absolute paths to relative
                sub_filter '=/' '=./';
                sub_filter '/seaweedfsstatic' './seaweedfsstatic';
                sub_filter_once off;
                */
			},
		},
		extraEnvironmentVars: {
			WEED_TIKV_ENABLED: "true",
			WEED_TIKV_PDADDRS: seaweedFsTikvSettings.pdAddressFQDN,
			WEED_MYSQL_ENABLED: "false",
			WEED_LEVELDB2_ENABLED: "false",
			/* with http DELETE, by default the filer would check whether a folder is empty.
         recursive_delete will delete all sub folders and files, similar to "rm -Rf" */
			WEED_FILER_OPTIONS_RECURSIVE_DELETE: "false",
			//   # directories under this folder will be automatically creating a separate bucket
			WEED_FILER_BUCKETS_FOLDER: "/buckets",
		} as ISeaweedfsOyelowo["filer"]["extraEnvironmentVars"] &
			Record<"WEED_TIKV_ENABLED" | "WEED_TIKV_PDADDRS", string>,
		s3: {
			// enableAuth: true,
			enabled: true,
			allowEmptyFolder: false,
			// Suffix of the host name, {bucket}.{domainName}
			domainName: "",
			// enable user & permission to s3 (need to inject to all services)
			skipAuthSecretCreation: false,
			auditLogConfig: {},
		},
	},
	s3: {
		enabled: false,
		// enableAuth: true,
		replicas: 1,
		port: 8333,
		metricsPort: 9327,
		allowEmptyFolder: false,
		// Suffix of the host name, {bucket}.{domainName}
		domainName: "",
		// enable user & permission to s3 (need to inject to all services)
		skipAuthSecretCreation: false,
		auditLogConfig: {},
		nodeSelector: 'sw-backend: "true"', // beta.kubernetes.io/arch: amd64
	},
	certificates: {
		commonName: "SeaweedFS CA",
		ipAddresses: [],
		keyAlgorithm: "rsa",
		keySize: 2048,
		duration: "2160h", // 90d
		renewBefore: "360h", // 15d
	},
};

// `http://${name}.${namespace}:${port}`;
const {
	repo,
	charts: { seaweedfs: { chart, version } },
} = helmChartsInfo.oyelowo;

export const seaweedfs = new k8s.helm.v3.Chart(
	chart,
	{
		chart,
		fetchOpts: {
			repo,
		},
		version,
		values: seaweedFsValues,
		namespace: namespaces.seaweedfs,
		// By default Release resource will wait till all created resources
		// are available. Set this to true to skip waiting on resources being
		// available.
		skipAwait: false,
	},
	{ provider: seaweedFsProvider },
);

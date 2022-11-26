import pc from "../../../generatedCrdsTs/index.js";
import { namespaces } from "../../types/ownTypes.js";
import { seaweedFsProvider } from "./settings.js";

const name = "seaweedfs-tikv";
export const seaweedFsTikvSettings = {
	name,
	namespace: namespaces.seaweedfs,
	pdPort: 2379,
	pdHost: `${name}-pd`,
	pdAddressFQDN: `${name}-pd:2379`,
};

export const seaweedFsTikvClusterFilerStore =
	new pc.pingcap.v1alpha1.TidbCluster(
		name,
		{
			metadata: {
				name: seaweedFsTikvSettings.name,
				namespace: namespaces.seaweedfs,
				// clusterName: "",
			},
			spec: {
				timezone: "UTC",
				configUpdateStrategy: "RollingUpdate",
				pvReclaimPolicy: "Retain",
				enableDynamicConfiguration: true,
				statefulSetUpdateStrategy: "RollingUpdate",
				pd: {
					baseImage: "pingcap/pd",
					service: {
						port: Number(seaweedFsTikvSettings.pdPort),
					},
					maxFailoverCount: 0,
					replicas: 3,
					requests: {
						storage: "10Gi",
					},
					storageClassName: "local-storage",
					config: `
                [dashboard]
                    internal-proxy = true
              ` as any,
				},
				tikv: {
					baseImage: "pingcap/tikv",
					maxFailoverCount: 0,
					storageClassName: "local-storage",
					replicas: 3,
					requests: {
						storage: "100Gi",
					},
					config: {},
				},
			},
		},
		{ provider: seaweedFsProvider },
	);

// Can also use TiDB as MySQL as filer store for Seaweedfs
/* const tidbClusterAutoScaler = new pc.pingcap.v1alpha1.TidbClusterAutoScaler('er', {
    apiVersion: 'pingcap.com/v1alpha1',
    kind: "TidbClusterAutoScaler",
    metadata: {
        name: "",
        namespace: "",
        clusterName: "",
        deletionGracePeriodSeconds: 120,
    },
    spec: {
        cluster: {
            clusterDomain: "",
            name: "",
            namespace: ""
        },
        tikv: {
            external: {
                maxReplicas: 5,
                endpoint: {
                    host: "",
                    path: "",
                    port: 2000
                },

            },
            scaleOutIntervalSeconds: 4,
            scaleInIntervalSeconds: 5,
        },
        // tidb: {}

    },

});
 */

import pc from '../../../generatedCrdsTs/index.js';
import { surrealdbSettings } from './surrealdb.js';

// TiKV acts as the persistent layer for surrealdb. Surrealdb also supports in-memory, file-based,
// foundationdb, rocksdb etc
const surrealDBTikvCluster = new pc.pingcap.v1alpha1.TidbCluster('tikv-cluster', {
    metadata: {
        name: surrealdbSettings.envVars.TIKV_NAME,
        namespace: surrealdbSettings.metadata.namespace,
        // clusterName: "",
    },
    spec: {
        timezone: 'UTC',
        configUpdateStrategy: 'RollingUpdate',
        pvReclaimPolicy: 'Retain',
        enableDynamicConfiguration: true,
        statefulSetUpdateStrategy: 'RollingUpdate',
        pd: {
            baseImage: 'pingcap/pd',
            service: {
                port: Number(surrealdbSettings.envVars.TIKV_PORT),
            },
            maxFailoverCount: 0,
            replicas: 3,
            requests: {
                storage: '10Gi',
            },
            storageClassName: 'local-storage',
            config: `
                [dashboard]
                    internal-proxy = true
              ` as any,
        },
        tikv: {
            baseImage: 'pingcap/tikv',
            maxFailoverCount: 0,
            storageClassName: 'local-storage',
            replicas: 3,
            requests: {
                storage: '100Gi',
            },
            config: {},
        },
    },
});

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

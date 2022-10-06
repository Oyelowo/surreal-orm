import pc from '../../../generatedCrdsTs/index.js';
// import { TidbCluster } from "../../../generatedCrdsTs/pingcap/v1alpha1/tidbCluster.js";

const k = new pc.pingcap.v1alpha1.TidbCluster('tikv-cluster', {
    spec: {
        timezone: 'UTC',
        configUpdateStrategy: 'RollingUpdate',
        pvReclaimPolicy: 'Retain',
        // enableDynamicConfiguration: true,
        // statefulSetUpdateStrategy: "RollingUpdate",
        pd: {
            baseImage: 'pingcap/pd',
            maxFailoverCount: 0,
            replicas: 3,
            requests: {
                storage: '10Gi',
            },
            // storageClassName: "local-storage"
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
// const k = new TidbCluster

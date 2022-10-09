import crds from '../../../generatedCrdsTs/index.js';
import { namespaces } from '../namespaces/util.js';
import k8s from '@pulumi/kubernetes';

// Export if you want to test
const rookCephConfigMap = new k8s.core.v1.ConfigMap('rook-ceph-override', {
    metadata: {
        name: 'rook-ceph-override',
        namespace: namespaces.rookCeph,
    },
    data: {
        config: `
        [global]
        osd_pool_default_size = 1
        mon_warn_on_pool_no_redundancy = false
        bdev_flock_retry = 20
        bluefs_buffered_io = false
        mon_data_avail_warn = 500M
        `,
    },
});

export const testCluster = new crds.ceph.v1.CephCluster('test-cluster', {
    metadata: {
        name: 'test-cluster',
        namespace: namespaces.rookCeph,
    },
    spec: {
        dataDirHostPath: '/var/lib/rook',
        cephVersion: {
            image: 'quay.io/ceph/ceph:v17',
            allowUnsupported: true,
        },

        mon: {
            count: 1,
            allowMultiplePerNode: true,
        },
        mgr: {
            count: 1,
            allowMultiplePerNode: true,
        },
        dashboard: {
            enabled: true,
        },
        crashCollector: {
            disable: true,
        },
        storage: {
            useAllNodes: true,
            useAllDevices: true,
        },
        healthCheck: {
            daemonHealth: {
                mon: {
                    interval: '45s',
                    timeout: '600s',
                },
            },
        },
        priorityClassNames: {
            all: 'system-node-critical',
            mgr: 'system-cluster-critical',
        },
        disruptionManagement: {
            managePodBudgets: true,
        },
    },
});

import crds from '../../../generatedCrdsTs/index.js';
import { namespaces } from '../namespaces/util.js';

const testCluster = new crds.ceph.v1.CephCluster('test-cluster', {
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

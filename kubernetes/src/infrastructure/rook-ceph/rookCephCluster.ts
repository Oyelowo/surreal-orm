import { IRookCephClusterRookCeph } from '../../../generatedHelmChartsTsTypes/rookCephClusterRookceph.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../../types/ownTypes.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { rookCephProvider } from './settings.js';

const rookCephClusterValues: DeepPartial<IRookCephClusterRookCeph> = {
    operatorNamespace: 'rook-ceph',
    toolbox: {
        enabled: false,
        image: 'rook/ceph:v1.10.2-15.ge3c280df1',
        tolerations: [],
        affinity: {},
        resources: {
            limits: {
                cpu: '500m',
                memory: '1Gi',
            },
            requests: {
                cpu: '100m',
                memory: '128Mi',
            },
        },
    },
    ingress: {
        dashboard: {},
    },
    cephBlockPools: [
        {
            name: 'ceph-blockpool',
            spec: {
                failureDomain: 'host',
                replicated: {
                    size: 3,
                },
            },
            storageClass: {
                enabled: true,
                name: 'ceph-block',
                isDefault: true,
                reclaimPolicy: 'Delete',
                allowVolumeExpansion: true,
                mountOptions: [],
                parameters: {
                    imageFormat: '2',
                    imageFeatures: 'layering',
                    'csi.storage.k8s.io/provisioner-secret-name': 'rook-csi-rbd-provisioner',
                    'csi.storage.k8s.io/provisioner-secret-namespace': 'rook-ceph',
                    'csi.storage.k8s.io/controller-expand-secret-name': 'rook-csi-rbd-provisioner',
                    'csi.storage.k8s.io/controller-expand-secret-namespace': 'rook-ceph',
                    'csi.storage.k8s.io/node-stage-secret-name': 'rook-csi-rbd-node',
                    'csi.storage.k8s.io/node-stage-secret-namespace': 'rook-ceph',
                    'csi.storage.k8s.io/fstype': 'ext4',
                },
            },
        },
    ],
    // TODO: Remove after testing. I only use seaweedfs as object store and blockstorage
    cephFileSystems: [
        {
            name: 'ceph-filesystem',
            spec: {
                metadataPool: {
                    replicated: {
                        // TODO: Delete object store or set to zero after testing. Seaweedfs is already used as object store
                        size: 3,
                    },
                },
                dataPools: [
                    {
                        failureDomain: 'host',
                        replicated: {
                            size: 3,
                        },
                        name: 'data0',
                    },
                ],
                metadataServer: {
                    activeCount: 1,
                    activeStandby: true,
                    resources: {
                        limits: {
                            cpu: '2000m',
                            memory: '4Gi',
                        },
                        requests: {
                            cpu: '1000m',
                            memory: '4Gi',
                        },
                    },
                    priorityClassName: 'system-cluster-critical',
                },
            },
            storageClass: {
                enabled: true,
                isDefault: false,
                name: 'ceph-filesystem',
                pool: 'data0',
                reclaimPolicy: 'Delete',
                allowVolumeExpansion: true,
                mountOptions: [],
                parameters: {
                    'csi.storage.k8s.io/provisioner-secret-name': 'rook-csi-cephfs-provisioner',
                    'csi.storage.k8s.io/provisioner-secret-namespace': 'rook-ceph',
                    'csi.storage.k8s.io/controller-expand-secret-name': 'rook-csi-cephfs-provisioner',
                    'csi.storage.k8s.io/controller-expand-secret-namespace': 'rook-ceph',
                    'csi.storage.k8s.io/node-stage-secret-name': 'rook-csi-cephfs-node',
                    'csi.storage.k8s.io/node-stage-secret-namespace': 'rook-ceph',
                    'csi.storage.k8s.io/fstype': 'ext4',
                },
            },
        },
    ],
    cephFileSystemVolumeSnapshotClass: {
        enabled: false,
        name: 'ceph-filesystem',
        isDefault: true,
        deletionPolicy: 'Delete',
        annotations: {},
        labels: {},
        parameters: {},
    },
    cephBlockPoolsVolumeSnapshotClass: {
        enabled: false,
        name: 'ceph-block',
        isDefault: false,
        deletionPolicy: 'Delete',
        annotations: {},
        labels: {},
        parameters: {},
    },
    cephObjectStores: [
        {
            name: 'ceph-objectstore',
            spec: {
                metadataPool: {
                    failureDomain: 'host',
                    replicated: {
                        // TODO: Delete object store or set to zero after testing. Seaweedfs is already used as object store
                        size: 3,
                    },
                },
                dataPool: {
                    failureDomain: 'host',
                    erasureCoded: {
                        dataChunks: 2,
                        codingChunks: 1,
                    },
                },
                preservePoolsOnDelete: true,
                gateway: {
                    port: 80,
                    resources: {
                        limits: {
                            cpu: '2000m',
                            memory: '2Gi',
                        },
                        requests: {
                            cpu: '1000m',
                            memory: '1Gi',
                        },
                    },
                    instances: 1,
                    priorityClassName: 'system-cluster-critical',
                },
                healthCheck: {
                    bucket: {
                        interval: '60s',
                    },
                },
            },
            storageClass: {
                enabled: true,
                name: 'ceph-bucket',
                reclaimPolicy: 'Delete',
                parameters: {
                    region: 'us-east-1',
                },
            },
        },
    ],
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        rookCephCluster: { chart, version },
    },
} = helmChartsInfo.rookCeph;

export const rookCephCluster = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: rookCephClusterValues,
        namespace: namespaces.rookCeph,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: rookCephProvider }
);

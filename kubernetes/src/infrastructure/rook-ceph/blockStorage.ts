import k8s from '@pulumi/kubernetes';
import crds from '../../../generatedCrdsTs/index.js';
import { namespaces } from '../../types/ownTypes.js';
import { rookCephProvider } from './settings.js';

export const rookCephBlockPool = new crds.ceph.v1.CephBlockPool(
    'rook-ceph-pool',
    {
        apiVersion: 'ceph.rook.io/v1',
        metadata: {
            name: 'replicapool',
            namespace: namespaces.rookCeph,
        },
        spec: {
            failureDomain: 'host',
            replicated: { size: 3, requireSafeReplicaSize: true },
        },
    },
    { provider: rookCephProvider }
);

const rookStorageClassParameters = {
    // clusterID is the namespace where the rook cluster is running
    clusterID: 'rook-ceph',
    // Ceph pool into which the RBD image shall be created
    pool: 'replicapool',

    // (optional) mapOptions is a comma-separated list of map options.
    // For krbd options refer
    // https:/"/docs.ceph.com/docs/master/man/8/rbd///kernel-rbd-krbd-options",
    // For nbd options refer
    // https:/"/docs.ceph.com/docs/master/man/8/rbd-nbd///options",
    // mapOptions: "lock_on_read,queue_depth=1024",

    // (optional) unmapOptions is a comma-separated list of unmap options.
    // For krbd options refer
    // https:/"/docs.ceph.com/docs/master/man/8/rbd///kernel-rbd-krbd-options",
    // For nbd options refer
    // https:/"/docs.ceph.com/docs/master/man/8/rbd-nbd///options",
    // unmapOptions: "force",

    // RBD image format. Defaults to "2".
    imageFormat: '2',

    // RBD image features. Available for imageFormat: ""2". CSI RBD currently supports only `layering` feature.",
    imageFeatures: 'layering',

    // The secrets contain Ceph admin credentials.
    'csi.storage.k8s.io/provisioner-secret-name': 'rook-csi-rbd-provisioner',
    'csi.storage.k8s.io/provisioner-secret-namespace': namespaces.rookCeph,
    'csi.storage.k8s.io/controller-expand-secret-name': 'rook-csi-rbd-provisioner',
    'csi.storage.k8s.io/controller-expand-secret-namespace': namespaces.rookCeph,
    'csi.storage.k8s.io/node-stage-secret-name': 'rook-csi-rbd-node',
    'csi.storage.k8s.io/node-stage-secret-namespace': namespaces.rookCeph,

    // Specify the filesystem type of the volume. If not specified, csi-provisioner
    // will set default as `ext4`. Note that `xfs` is not recommended due to potential deadlock
    // in hyperconverged settings where the volume is mounted on the same node as the osds.
    'csi.storage.k8s.io/fstype': 'ext4',
};
// // Change "rook-ceph" provisioner prefix to match the operator namespace if needed
const rookCephProvisioner = `${namespaces.rookCeph}.rbd.csi.ceph.com`;
// const rookCephProvisioner = 'rook-ceph.rbd.csi.ceph.com'
export const rookCephBlockStorage = new k8s.storage.v1.StorageClass(
    'rook-ceph',
    {
        apiVersion: 'storage.k8s.io/v1',
        metadata: {
            name: 'replicapool',
            // namespace: namespaces.rookCeph
        },
        provisioner: rookCephProvisioner,
        parameters: rookStorageClassParameters,
        reclaimPolicy: 'Retain',
        /* 
    # Optional, if you want to add dynamic resize for PVC.
# For now only ext3, ext4, xfs resize support provided, like in Kubernetes itself.
    */
        allowVolumeExpansion: true,
    },
    { provider: rookCephProvider }
);

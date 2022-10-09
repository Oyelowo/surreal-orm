import k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';

export const rookCephConfigMap = new k8s.core.v1.ConfigMap('rook-ceph-override', {
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

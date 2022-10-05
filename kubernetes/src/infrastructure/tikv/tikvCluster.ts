import { ITidbclusterpingcap } from '../../../generatedHelmChartsTsTypes/tidbClusterPingcap.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { tikvProvider } from './settings.js';

const tikvOperatValues: DeepPartial<ITidbclusterpingcap> = {
    timezone: 'UTC',
    pvReclaimPolicy: 'Retain',
    enableConfigMapRollout: true,
    pd: {
        replicas: 3,
        resources: {
            requests: {
                storage: '10Gi',
            },
        },
        // storageClassName: 'local-storage'
        /* # Please refer to https://github.com/pingcap/pd/blob/master/conf/config.toml for the default
# pd configurations (change to the tags of your pd version),
# just follow the format in the file and configure in the 'config' section
# as below if you want to customize any configuration.
# Please refer to https://pingcap.com/docs/stable/reference/configuration/pd-server/configuration-file 
# (choose the version matching your pd) for detailed explanation of each parameter.
*/
        config: `
    [log]
    level = "info"
    [replication]
    location-labels = ["region", "zone", "rack", "host"]

        `,
    },
    tikv: {
        replicas: 3,
        resources: {
            requests: {
                storage: '100Gi',
            },
        },
        /*
                  # storageClassName is a StorageClass provides a way for administrators to describe the "classes" of storage they offer.
          # different classes might map to quality- of - service levels, or to backup policies,
          # or to arbitrary policies determined by the cluster administrators.
          # refer to https://kubernetes.io/docs/concepts/storage/storage-classes
          */
        // storageClassName: 'local-storage'
        /* 
          # Please refer to https://github.com/tikv/tikv/blob/master/etc/config-template.toml for the default
  # tikv configurations (change to the tags of your tikv version),
  # just follow the format in the file and configure in the 'config' section
  # as below if you want to customize any configuration.
  # Please refer to https://pingcap.com/docs/stable/reference/configuration/tikv-server/configuration-file
  # (choose the version matching your tikv) for detailed explanation of each parameter.
        */
        config: `
            log-level = "info"
  # # Here are some parameters you MUST customize (Please configure in the above "tikv.config" section):
  #
  # [readpool.coprocessor]
  #   # Normally these three parameters should be tuned to 80% of "tikv.resources.limits.cpu", for example: 10000m -> 8
  #   high-concurrency = 8
  #   normal-concurrency = 8
  #   low-concurrency = 8
  #
  # # For TiKV v2.x:
  # [rocksdb.defaultcf]
  # ## block-cache used to cache uncompressed blocks, big block-cache can speed up read.
  # ## in normal cases should tune to 30%-50% "tikv.resources.limits.memory"
  # # block-cache-size = "1GB"
  #
  # [rocksdb.writecf]
  # ## in normal cases should tune to 10%-30% "tikv.resources.limits.memory"
  # # block-cache-size = "256MB"
  #
  # # From TiKV v3.0.0 on, you do not need to configure
  # #  [rocksdb.defaultcf].block-cache-size and [rocksdb.writecf].block-cache-size.
  # # Instead, configure [storage.block-cache] as below:
  # [storage.block-cache]
  #   shared = true
  #
  #   # Normally it should be tuned to 30%-50% of "tikv.resources.limits.memory", for example: 32Gi -> 16GB
  #   capacity = "1GB"
  # Note that we can't set raftstore.capacity in config because it will be overridden by the command line parameter,
  # we can only set capacity in tikv.resources.limits.storage.
        `,
    },
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        tikvCluster: { chart, version },
    },
} = helmChartsInfo.pingcap;

export const tikvCluster = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: tikvOperatValues,
        namespace: namespaces.tikvAdmin,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: tikvProvider }
);

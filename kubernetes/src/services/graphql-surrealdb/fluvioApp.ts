import crds from '../../../generatedCrdsTs/index.js';
import { surrealdbSettings } from './surrealdb.js';
crds.fluvio.v2.Topic;

export const userLocationTopic = new crds.fluvio.v2.Topic('', {
    apiVersion: 'fluvio.infinyon.com/v2',
    kind: 'Topic',
    metadata: {
        name: 'user-location',
        namespace: surrealdbSettings.metadata.namespace,
    },
    spec: {
        replicas: 1,
        storage: {
            maxPartitionSize: 3,
            segmentSize: 3,
        },
    },
});

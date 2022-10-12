import crds from '../../../generatedCrdsTs/index.js';
import { graphqlSurrealdb } from './app.js';
import { surrealdbSettings } from './surrealdb.js';
crds.fluvio.v2.Topic;

export const userLocationTopic = new crds.fluvio.v2.Topic(
    'test-user-location-topic',
    {
        apiVersion: 'fluvio.infinyon.com/v2',
        kind: 'Topic',
        metadata: {
            name: 'test-user-location',
            namespace: surrealdbSettings.metadata.namespace,
        },
        spec: {
            replicas: 1,
            storage: {
                maxPartitionSize: 3,
                segmentSize: 3,
            },
        },
    },
    { provider: graphqlSurrealdb.getProvider() }
);

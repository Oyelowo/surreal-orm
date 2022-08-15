import * as linode from '@pulumi/linode';
import { Environment } from '../../resources/types/ownTypes.js';
import { LINODE_REGION_ID } from './shared.js';

export const environment: Environment = 'production';
export const linodeLkeCluster = new linode.LkeCluster(`lke-cluster-${environment}`, {
    k8sVersion: '1.23',
    label: `lke-cluster-${environment}`,
    controlPlane: {
        // highAvailability: true,
    },
    pools: [
        {
            count: 3,
            type: 'g6-standard-2',
        },
    ],
    region: LINODE_REGION_ID,
    tags: [environment],
});

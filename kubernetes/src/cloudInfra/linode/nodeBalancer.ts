import * as linode from '@pulumi/linode';
import { environment } from './lkeCluster.js';
import { LINODE_REGION_ID } from './shared.js';

export const linodeNodeBalancer = new linode.NodeBalancer(`node-balancer-${environment}`, {
    region: LINODE_REGION_ID,
    label: `node-balancer-${environment}`,
    tags: [environment],
    // clientConnThrottle: 0,
});
linodeNodeBalancer.hostname;

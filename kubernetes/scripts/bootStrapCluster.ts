import { promptEnvironmentSelection } from './utils/shared.js';
import { setupCluster } from './utils/setupCluster.js';

/* 
Expects that the cluster is already running and in user's local
machine context
*/

async function main() {
    const { environment } = await promptEnvironmentSelection();

    await setupCluster(environment);
}

main().catch((e) => `Failed to bootstrap. Error: ${e}`);

import { KubeObject } from './utils/kubeObject/kubeObject.js';

/* 
Does not handle sealed secret generation/syncing
*/

import yargs from 'yargs';
import { ENVIRONMENTS_ALL } from './utils/shared.js';

export const ARGV_ENVIRONMENTS = yargs(process.argv.slice(2))
    .options({
        environment: {
            alias: 'e',
            choices: ENVIRONMENTS_ALL,
            describe: "The environment you're generating the manifests for.",
            demandOption: true,
        },
    })
    .parseSync();

async function main() {
    const kubeObject = new KubeObject(ARGV_ENVIRONMENTS.environment);
    await kubeObject.generateManifests();
}

await main();

import { getAllKubeManifestsInfo } from './utils/shared';
/* 
Does not handle sealed secret generation/syncing
*/

import yargs from 'yargs';
import { generateManifests } from './utils/generateManifests';
import { getImageTagsFromDir } from './utils/getImageTagsFromDir';
import { ENVIRONMENTS_ALL } from './utils/shared';

export const ARGV = yargs(process.argv.slice(2))
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
    const imageTags = await getImageTagsFromDir();
    await generateManifests({
        environment: ARGV.environment,
        imageTags,
        allManifestsInfo: getAllKubeManifestsInfo(ARGV.environment)
    });
}

main().catch((e) => console.log('e', e));

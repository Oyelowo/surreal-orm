import yargs from 'yargs'
import { generateManifests } from './utils/generateManifests'
import { getImageTagsFromDir } from './utils/getImageTagsFromDir'
import { ENVIRONMENTS_ALL } from './utils/sealedSecrets'

export const ARGV = yargs(process.argv.slice(2))
    .options({
        environment: {
            alias: 'e',
            choices: ENVIRONMENTS_ALL,
            describe: "The environment you're generating the manifests for.",
            demandOption: true,
        },
    })
    .parseSync()

async function main() {
    const imageTags = await getImageTagsFromDir()
    await generateManifests({
        environment: ARGV.environment,
        imageTags,
    })
}

main()

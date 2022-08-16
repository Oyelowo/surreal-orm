import path from 'node:path';
import yargs from 'yargs';
import { Environment } from '../../../kubernetes/src/resources/types/ownTypes.js';

export const ENVIRONMENTS: Environment[] = ['production', 'staging', 'development'];

export const ARGV_ENVIRONMENTS = yargs(process.argv.slice(2))
    .options({
        environment: {
            alias: 'e',
            choices: ENVIRONMENTS,
            describe: "The environment you're generating the manifests for.",
            demandOption: true,
        },
    })
    .parseSync();


const mainDir = process.cwd();
export const tsConfigPath = path.join(mainDir, 'tsconfig.pulumi.json');

import path from 'node:path';
import yargs from 'yargs';
import { getMainBaseDir } from '../../src/shared/directoriesManager.js';
import { ENVIRONMENTS_ALL } from '../utils/shared.js';

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

const mainDir = getMainBaseDir();

export const tsConfigPath = path.join(mainDir, 'tsconfig.pulumi.json');

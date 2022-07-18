// NOTE: This file is intended to be called via shell command
// line because the imported secrets may not exist during the course of
// running the code
import R from 'ramda';
import sh from 'shelljs';
import yargs from 'yargs';
import { ENVIRONMENTS_ALL } from '../utils/shared';
import { secretRecord } from './getSecretsForApp';
import { secretsSample } from './secretsSample';
import { getPlainSecretsContent } from './utils';
import { getPlainSecretInputFilePath } from './utils';

const ARGV = yargs(process.argv.slice(2))
    .options({
        environment: {
            alias: 'e',
            choices: ENVIRONMENTS_ALL,
            describe: "The environment you're generating the manifests for.",
            demandOption: true,
        },
    })
    .parseSync();

// Usage ./mergeSecrets --environment=<local>
function main() {
    const { environment } = ARGV;
    const existingSecrets = secretRecord[environment] ?? {};
    const secrets = R.mergeDeepLeft(existingSecrets, secretsSample);
    const filePath = getPlainSecretInputFilePath(environment);
    const content = getPlainSecretsContent({ environment, secrets });
    sh.exec(`echo ${content} > ${filePath}`);
}

main();

import { getMainBaseDir } from '../../resources/shared/manifestsDirectory';
/*
TODO: ADD INSTRUCTION HERE
*/
import c from 'chalk';
import path from 'path';
import sh from 'shelljs';
import { Environment } from '../../resources/types/own-types';
import { getPlainSecretsConfigFilesBaseDir } from '../../resources/shared/manifestsDirectory';
import { secretsSample } from './secretsSample';
import { getPlainSecretInputFilePath, PLAIN_SECRETS_CONFIGS_DIR, getPlainSecretsContent } from './utils';

const ENVIRONMENTS: Environment[] = ['local', 'development', 'staging', 'production'];

export function syncSecretsTsFiles() {
    ENVIRONMENTS.forEach(createSecretsConfigFile);
    sh.exec(`npx prettier --write ${PLAIN_SECRETS_CONFIGS_DIR}`);
}

// main()

export function clearPlainInputTsSecretFilesContents() {
    const removeSecret = (env: Environment) => sh.rm('-rf', getPlainSecretInputFilePath(env));
    ENVIRONMENTS.forEach(removeSecret);

    syncSecretsTsFiles();
}

function createSecretsConfigFile(environment: Environment) {
    // console.log(c.blueBright`Generating/Updating secrets for environment : ${environment}`);

    const plainSecretsPath = getPlainSecretInputFilePath(environment);
    const content = getPlainSecretsContent({
        environment,
        secrets: secretsSample,
    });

    // TODO: This check can be improved to check the serialized content against the sample
    const secretsContent = sh.cat(plainSecretsPath)?.stdout?.trim();
    const secretsExists = !!secretsContent;

    const createSecrets = () => {
        console.log(c.blueBright`Creating secrets for environment : ${environment} at ${plainSecretsPath}`);
        sh.mkdir(path.dirname(plainSecretsPath));
        sh.touch(plainSecretsPath);
        sh.exec(`echo ${content} > ${plainSecretsPath}`);
    };

    const mergeSecrets = () => {
        console.log(c.blueBright`Merging secrets for environment : ${environment} at ${plainSecretsPath}`);
        const tsNoCheckMsg = '// @ts-nocheck';
        // Disable typechecking first so that it can do the merging without typescript erroring out
        sh.exec(`echo "$(echo '${tsNoCheckMsg}'; cat ${plainSecretsPath})" > ${plainSecretsPath}`);
        const mergeSecretsPath = path.join(getMainBaseDir(), 'scripts', 'secretsManagement', 'mergeSecretsCli.ts');

        const exec = sh.exec(`npx ts-node ${mergeSecretsPath} --environment=${environment}`, { silent: true });

        if (!exec.stderr.includes('Error: Cannot find module')) {
            console.warn(
                c.redBright(
                    `
            Make sure the secret file at "${plainSecretsPath}" does not have an invalid key. Remove it if it has a key that does not exist in the secretsSample file. 
            \n
            Error:`,
                    exec.stderr
                )
            );
        }
    };

    secretsExists ? mergeSecrets() : createSecrets();
}

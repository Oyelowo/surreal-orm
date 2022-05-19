/*
TODO: ADD INSTRUCTION HERE
*/
import c from 'chalk';
import path from 'path';
import sh from 'shelljs';
import { Environment } from '../../resources/shared/types/own-types';
import { getPlainSecretsConfigFilesBaseDir } from './../../resources/shared/manifestsDirectory';
import { secretsSample } from './secretsSample';

const ENVIRONMENTS: Environment[] = ['local', 'development', 'staging', 'production'];
const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();

type PlainInputSecretsFilePath = `${typeof PLAIN_SECRETS_CONFIGS_DIR}/${Environment}.ts`;

export function setupPlainSecretTSFiles() {
    ENVIRONMENTS.forEach(createSecretsConfigFile);
    sh.exec(`npx prettier --write ${PLAIN_SECRETS_CONFIGS_DIR}`);
}

export function clearPlainInputTsSecretFilesContents() {
    const removeSecret = (env: Environment) => sh.rm('-rf', getPlainSecretInputFilePath(env));

    ENVIRONMENTS.forEach(removeSecret);

    setupPlainSecretTSFiles();
}

export function getPlainSecretInputFilePath(environment: Environment): PlainInputSecretsFilePath {
    return `${PLAIN_SECRETS_CONFIGS_DIR}/${environment}.ts`;
}

export type Secrets = typeof secretsSample;

async function createSecretsConfigFile(environment: Environment) {
    const filePath = getPlainSecretInputFilePath(environment);
    const content = getPlainSecretsContent({
        environment,
        secrets: secretsSample,
    });

    const tsNoCheckMsg = '// @ts-nocheck';
    // TODO: This check can be improved to check the serialized content against the sample
    const secretsContent = sh.cat(filePath)?.stdout?.trim();
    const secretsExists = !!secretsContent && !secretsContent.includes(tsNoCheckMsg);

    const createSecrets = () => {
        sh.mkdir(path.dirname(filePath));
        sh.touch(filePath);
        sh.exec(`echo "$(echo '${tsNoCheckMsg}'; cat ${filePath})" > ${filePath}`);
        sh.exec(`echo ${content} > ${filePath}`);
    };

    const mergeSecrets = () => {
        const exec = sh.exec('npx ts-node ./scripts/secretsManagement/mergeSecrets.ts', { silent: true });
        if (!exec.stderr.includes('Error: Cannot find module')) {
            console.error(c.redBright(exec.stderr));
        }
    };

    secretsExists ? mergeSecrets() : createSecrets();
}

export function getPlainSecretsContent({ environment, secrets }: { environment: Environment; secrets: Secrets }) {
    const thisFileRelativeDir = __dirname.split('/').slice(-2).join('/');
    const thisFileName = path.basename(__filename).slice(0, -3);
    const SECRETS_TYPE = 'Secrets' as const; // This should be same as the secrets type above

    return JSON.stringify(`
    import {${SECRETS_TYPE}} from "../${thisFileRelativeDir}/${thisFileName}";
    
     export const SECRET_${environment.toUpperCase()}: ${SECRETS_TYPE} = ${JSON.stringify(secrets)};
    `);
}

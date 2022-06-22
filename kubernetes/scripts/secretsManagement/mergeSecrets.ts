// NOTE: This file is intended to be called via shell command
// line because the imported secrets may not exist during the course of
// running the code
import R from 'ramda';
import sh from 'shelljs';
import { Environment } from '../../resources/types/own-types';
import { ENVIRONMENTS_ALL } from '../utils/sealedSecrets';
import { secretRecord } from './getSecretsForApp';
import { secretsSample } from './secretsSample';
import { getPlainSecretInputFilePath, getPlainSecretsContent } from './setupSecrets';

function mergeWithExistingSecrets(environment: Environment) {
    const existingContent = secretRecord[environment] ?? {};
    const secrets = R.mergeDeepLeft(existingContent, secretsSample);
    const filePath = getPlainSecretInputFilePath(environment);
    const content = getPlainSecretsContent({ environment, secrets });

    sh.exec(`echo ${content} > ${filePath}`);
}

function main() {
    ENVIRONMENTS_ALL.forEach(mergeWithExistingSecrets);
}
main();

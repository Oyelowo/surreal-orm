import sh from 'shelljs';
import { getEnvVarsForKubeManifestGenerator, getKubeBuildEnvVarsSchema, getKubeBuildEnvVarsSample } from '../../src/resources/types/environmentVariables.js';
import * as dotenv from 'dotenv'; // see https://github.com/motdotla/dotenv#how-do-i-use-dotenv-with-import
import { Environment } from '../../src/resources/types/ownTypes.js';
import { getPlainSecretsConfigFilesBaseDir } from '../../src/resources/shared/directoriesManager.js';
import { ARGV_ENVIRONMENTS, ENVIRONMENTS_ALL } from '../utils/shared.js';
import path from 'node:path';
import * as R from 'ramda';
import _ from 'lodash';


const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();

const getSecretPath = (e: Environment) => path.join(PLAIN_SECRETS_CONFIGS_DIR, `.env.${e}`)

const getSecretJsonObject = (environment: Environment) => {
    const envPath = getSecretPath(environment);

    const existingEnvSecret = dotenv.parse(sh.exec(`cat ${envPath}`, { silent: true }).stdout.trim());
    return existingEnvSecret;
};





export const kubeBuildEnvVarsManager = {
    // getEnvVars = () => {
    //     return getEnvVarsForKubeManifestGenerator()
    // };

    resetValues: (environment: Environment): void => {
        sh.echo(`Empting secret JSON config for ${environment}`);
        sh.mkdir('-p', PLAIN_SECRETS_CONFIGS_DIR);
        const envPath = getSecretPath(environment);

        const kubeBuildEnvVarsSample = getKubeBuildEnvVarsSample({ environment })
        sh.exec(`echo '${JSON.stringify(kubeBuildEnvVarsSample)}' > ${envPath}`);
        sh.exec(`npx prettier --write ${envPath}`);
    },

    syncAll: (): void => {
        ENVIRONMENTS_ALL.forEach((environment) => {
            sh.echo(`Syncing Secret JSON config for ${environment}`);
            sh.mkdir('-p', PLAIN_SECRETS_CONFIGS_DIR);

            const envPath = getSecretPath(environment);
            const existingEnvSecret = getSecretJsonObject(environment)
            // const existingEnvSecret = process.env

            if (_.isEmpty(existingEnvSecret)) sh.touch(envPath);

            // Allows us to only get valid keys out, so we can parse the merged secrets out.
            const secretsSchema = getKubeBuildEnvVarsSchema({ allowEmptyValues: true });
            const kubeBuildEnvVarsSample = getKubeBuildEnvVarsSample({ environment })
            // Parse the object to filter out stale keys in existing local secret configs
            // This also persists the values of existing secrets
            const mergedObject = R.mergeDeepLeft(existingEnvSecret, kubeBuildEnvVarsSample);
            mergedObject.ENVIRONMENT = environment

            const envVars = secretsSchema.parse(mergedObject);

            const updatedEnvVars = Object.entries(envVars).map(([name, value]) => `${name}=${value}`).join('\n');

            sh.exec(`echo '${updatedEnvVars}' > ${envPath}`);
        });
    }

}

// kubeBuildEnvVarsManager.syncAll()
import sh from 'shelljs';
import path from 'node:path';
import _ from 'lodash';
import * as R from 'ramda';
import {
    getKubeBuildEnvVarsSchema,
    getKubeBuildEnvVarsSample,
    KubeBuildEnvVars,
    getEnvVarsForKubeManifests,
} from '../../src/resources/types/environmentVariables.js';
import { getMainBaseDir } from '../../src/resources/shared/directoriesManager.js';

const envPath = path.join(getMainBaseDir(), `.env`);

export const kubeBuildEnvVarsManager = {
    resetValues: (): void => {
        sh.echo(`Emptying dot env values`);

        const kubeBuildEnvVarsSample = getKubeBuildEnvVarsSample();

        const dotEnvConfig = generateDotEnvFile(kubeBuildEnvVarsSample)
        sh.exec(`echo '${dotEnvConfig}' > ${envPath}`);
        sh.exec(`npx prettier --write ${envPath}`);
    },

    syncAll: (): void => {
        sh.echo(`Syncing Secret .env config`);

        const existingEnvSecret = getEnvVarsForKubeManifests({ check: false })

        if (_.isEmpty(existingEnvSecret)) sh.touch(envPath);

        // Allows us to only get valid keys out, so we can parse the merged secrets out.
        const secretsSchema = getKubeBuildEnvVarsSchema({ allowEmptyValues: true });
        const kubeBuildEnvVarsSample = getKubeBuildEnvVarsSample();

        // Parse the object to filter out stale keys in existing local secret configs
        // This also persists the values of existing secrets
        const mergedObject = R.mergeDeepLeft(existingEnvSecret, kubeBuildEnvVarsSample);

        const envVars = secretsSchema.parse(mergedObject) as KubeBuildEnvVars;

        const updatedEnvVars = generateDotEnvFile(envVars);

        sh.exec(`echo '${updatedEnvVars}' > ${envPath}`);
    },
};


function generateDotEnvFile(envVars: KubeBuildEnvVars) {
    return Object.entries(envVars)
        .map(([name, value]) => `${name}=${value}`)
        .join('\n');
}


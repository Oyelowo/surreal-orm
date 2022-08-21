
import * as dotenv from 'dotenv';

import {
    appEnvironmentsSchema,
    Environment,
    EnvVarsCommon,
    InfrastructureName,
    ResourceCategory,
    ServiceName,
    TServices,
} from '../src/resources/types/ownTypes.js';
import { Simplify, SnakeCase } from 'type-fest';
import z from 'zod';
import _ from 'lodash';
import sh from 'shelljs';
import path from 'node:path';
import * as R from 'ramda';
import { getMainBaseDir } from '../src/resources/shared/directoriesManager.js';
import { getKubeBuildEnvVarsSample, KubeBuildEnvVars } from '../src/resources/types/environmentVariables.js';
import { ENVIRONMENTS_ALL } from './utils/shared.js';


// export const getKubeBuildEnvVarsSchema = ({
//     allowEmptyValues,
//     requireValues,
// }: {

// }) => {

//     return z.object(kubeBuildEnvVarsSchema);
// };

type Option = {
    // allowEmptyValues: boolean;
    requireValues?: boolean;
};
export const validateEnvVar = (data: unknown, option: Option): KubeBuildEnvVars => {
    const { requireValues } = option ?? {}
    // This is done to allow us sync local .env files.
    // When parsing to sync the env var names/keys, we want the values to allow empty
    // const string = z.string().min(allowEmptyValues ? 0 : 1)
    const string = z.string().min(requireValues ? 1 : 0)
    const kubeBuildEnvVarsSample = getKubeBuildEnvVarsSample();

    const kubeBuildEnvVarsSchema: Record<keyof KubeBuildEnvVars, z.ZodOptional<z.ZodString> | z.ZodString> = _.mapValues(
        kubeBuildEnvVarsSample,
        (_) => requireValues ? string : string.optional()
    );
    kubeBuildEnvVarsSchema.ENVIRONMENT = requireValues ? appEnvironmentsSchema as any : appEnvironmentsSchema.optional();

    const schema = z.object(kubeBuildEnvVarsSchema);
    return schema.parse(data) as KubeBuildEnvVars;
};

const envPath = path.join(getMainBaseDir(), `.env.local`);


// eslint-disable-next-line unicorn/no-static-only-class
export class KubeBuildEnvVarsManager {
    static getEnvVars() {
        // dotenv.config({ override: false });
        const env = appEnvironmentsSchema.parse(process.env.ENVIRONMENT)
        const envy = KubeBuildEnvVarsManager.#getEnvVarsForEnv(env);
        // console.log("XXXC", envy)
        return envy
    }

    static #getEnvVarsForEnv(env: Environment): KubeBuildEnvVars {
        // dotenv.config({ override: false });
        // const env = appEnvironmentsSchema.parse(process.env.ENVIRONMENT)
        dotenv.config({ path: path.join(getMainBaseDir(), `.env.${env}`), override: true });
        const envy = validateEnvVar(env, { requireValues: false });
        // console.log("XXXC", envy)
        return envy
    }

    static resetValues = (): void => {
        sh.echo(`Emptying dot env values`);

        const kubeBuildEnvVarsSample = getKubeBuildEnvVarsSample();

        const dotEnvConfig = generateDotEnvFile(kubeBuildEnvVarsSample);
        sh.exec(`echo '${dotEnvConfig}' > ${envPath}`);
        sh.exec(`npx prettier --write ${envPath}`);
    }

    static syncAll = (): void => {

        sh.echo(`Syncing Secret .env config`);
        ENVIRONMENTS_ALL.forEach(env => {

            // dotenv.config({ path: path.join(getMainBaseDir(), `.env.${env}`), override: true });
            const existingEnvSecret = KubeBuildEnvVarsManager.#getEnvVarsForEnv(env);

            if (_.isEmpty(existingEnvSecret)) sh.touch(envPath);

            // Allows us to only get valid keys out, so we can parse the merged secrets out.
            // const secretsSchema = validateEnvVar({ requireValues: true });
            const kubeBuildEnvVarsSample = getKubeBuildEnvVarsSample();

            // Parse the object to filter out stale keys in existing local secret configs
            // This also persists the values of existing secrets
            const mergedObject = R.mergeDeepLeft(existingEnvSecret, kubeBuildEnvVarsSample);

            const envVars = validateEnvVar(mergedObject, { requireValues: true });

            const updatedEnvVars = generateDotEnvFile(envVars);

            sh.exec(`echo '${updatedEnvVars}' > ${envPath}`);
        })
    }
};

function generateDotEnvFile(envVars: KubeBuildEnvVars) {
    return Object.entries(envVars)
        .map(([name, value]) => `${name}=${value}`)
        .join('\n');
}

// const env = getEnvVarsForKubeManifests();
// const ENVIRONMENT_KEY: keyof Pick<KubeBuildEnvVars, 'ENVIRONMENT'> = 'ENVIRONMENT';
// export function getEnvVarsForScript({ environment }: { environment: Environment }) {
//     const imageEnvVarSetterForPulumi = Object.entries(process.env)
//         .map(([k, v]) => `export ${k}=${v}`)
//         .join(' ');
//     return `
//       ${imageEnvVarSetterForPulumi}
//       export ${ENVIRONMENT_KEY}=${environment}
//   `;
// }


// console.log("PPP", KubeBuildEnvVarsManager.getEnvVars())
console.log("PPP", KubeBuildEnvVarsManager.syncAll())

import path from 'path';
import { getPlainSecretsConfigFilesBaseDir } from '../../resources/shared/manifestsDirectory';
import { Environment } from '../../resources/types/own-types';
import { secretsSample } from './secretsSample';

// import { SECRET_DEVELOPMENT } from '../../.secrets/development';
// import { SECRET_LOCAL } from '../../.secrets/local';
// import { SECRET_PRODUCTION } from '../../.secrets/production';
// import { SECRET_STAGING } from '../../.secrets/staging';
// import { Environment, ResourceName } from '../../resources/types/own-types';

// export const secretRecord: Record<Environment, Secrets> = {
//     production: SECRET_PRODUCTION,
//     staging: SECRET_STAGING,
//     development: SECRET_DEVELOPMENT,
//     local: SECRET_LOCAL,
// };

// type AppSecrets<App extends ResourceName> = typeof secretRecord[Environment][App];

// export function getSecretsForResource<Resource extends ResourceName>(
//     resourceName: Resource,
//     environment: Environment
// ): AppSecrets<Resource> {
//     return secretRecord[environment][resourceName];
// }

export const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();
export function getPlainSecretInputFilePath(environment: Environment): string {
    return path.join(PLAIN_SECRETS_CONFIGS_DIR, `${environment}.ts`);
}

export type Secrets = typeof secretsSample;

export function getPlainSecretsContent({ environment, secrets }: { environment: Environment; secrets: Secrets }) {
    const thisFileRelativeDir = __dirname.split('/').slice(-2).join('/');
    const thisFileName = path.basename(__filename).slice(0, -3);
    const SECRETS_TYPE = 'Secrets' as const; // This should be same as the secrets type above

    return JSON.stringify(`
    import {${SECRETS_TYPE}} from "../${thisFileRelativeDir}/${thisFileName}";
    
     export const SECRET_${environment.toUpperCase()}: ${SECRETS_TYPE} = ${JSON.stringify(secrets)};
    `);
}

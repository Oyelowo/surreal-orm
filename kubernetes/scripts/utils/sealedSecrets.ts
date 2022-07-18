import p from 'path';
import { getGeneratedEnvManifestsDir } from '../../resources/shared/manifestsDirectory';
import { Environment, ResourceName } from '../../resources/types/own-types';
import { getKubeResourceTypeInfo } from './shared';

export const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = 'sealed-secrets';

export function getSecretPathsInfo({ unsealedSecretFilePath }: { unsealedSecretFilePath: string }) {
    const appManifestsDir = p.dirname(unsealedSecretFilePath);
    // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
    // and we want as basedir: kubernetes/manifests/generated/production/applications/graphql-mongo
    const appBaseDir = p.join(appManifestsDir, '..');
    const unsealedSecretFileName = p.basename(unsealedSecretFilePath);

    // CONSIDER?: Get this as an argument to the function whch will be prompted on command start
    // if (secretsToUpdate.inclues(unsealedSecretFileName)) {
    // }

    const sealedSecretDir = p.join(appBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
    const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-${unsealedSecretFileName}`);
    return {
        sealedSecretDir,
        sealedSecretFilePath,
        // sealedSecretsControllerName: SEALED_SECRETS_CONTROLLER_NAME,
    } as const;
}

export function getSecretManifestsPaths(environment: Environment) {
    return getKubeResourceTypeInfo({
        resourceType: "Service",
        environment,
    }).map(info => info.path);
}




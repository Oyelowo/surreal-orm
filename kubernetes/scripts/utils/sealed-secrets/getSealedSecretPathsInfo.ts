import p from 'path';
import { getGeneratedEnvManifestsDir } from '../../../resources/shared/manifestsDirectory';
import { ResourceName } from '../../../resources/types/own-types';

export const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = 'sealed-secrets';

export function getSealedSecretPathsInfo({ kubeSecretManifestPath }: { kubeSecretManifestPath: string }) {
    const appManifestsDir = p.dirname(kubeSecretManifestPath);
    // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
    // and we want as basedir: kubernetes/manifests/generated/production/applications/graphql-mongo
    const appBaseDir = p.join(appManifestsDir, '..');
    const unsealedSecretFileName = p.basename(kubeSecretManifestPath);

    const sealedSecretDir = p.join(appBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
    const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-${unsealedSecretFileName}`);
    return {
        sealedSecretDir,
        sealedSecretFilePath,
        // sealedSecretsControllerName: SEALED_SECRETS_CONTROLLER_NAME,
    } as const;
}



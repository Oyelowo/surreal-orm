import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

export const sealedSecretsResourceName: ResourceName = 'sealed-secrets';
export const sealedSecretsProvider = getResourceProvider({
    outputDirectory: `infrastructure/sealed-secrets`,
    environment: ENVIRONMENT,
});

import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const sealedSecretsResourceName: ResourceName = 'tikv';
export const sealedSecretsProvider = getResourceProvider({
    outputDirectory: `infrastructure/tikv`,
    environment: ENVIRONMENT,
});

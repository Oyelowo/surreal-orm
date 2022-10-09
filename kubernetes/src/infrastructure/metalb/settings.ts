import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const metalbResourceName: ResourceName = 'metalb';
export const metalbProvider = getResourceProvider({
    outputDirectory: `infrastructure/metalb`,
    environment: ENVIRONMENT,
});

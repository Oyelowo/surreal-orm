import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const ciliumResourceName: ResourceName = 'cilium';
export const ciliumProvider = getResourceProvider({
    outputDirectory: `infrastructure/cilium`,
    environment: ENVIRONMENT,
});

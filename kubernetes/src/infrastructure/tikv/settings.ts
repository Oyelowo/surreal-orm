import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const tikvResourceName: ResourceName = 'tikv';
export const tikvProvider = getResourceProvider({
    outputDirectory: `infrastructure/tikv`,
    environment: ENVIRONMENT,
});

import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const fluvioSysResourceName: ResourceName = 'fluvio-sys';
export const fluvioSysProvider = getResourceProvider({
    outputDirectory: `infrastructure/fluvio-sys`,
    environment: ENVIRONMENT,
});

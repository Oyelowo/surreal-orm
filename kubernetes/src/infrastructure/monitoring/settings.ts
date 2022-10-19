import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const monitoringProvider = getResourceProvider({
    outputDirectory: `infrastructure/monitoring`,
    environment: ENVIRONMENT,
});

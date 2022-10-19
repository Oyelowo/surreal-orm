import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const harborProvider = getResourceProvider({
    outputDirectory: `infrastructure/harbor`,
    environment: ENVIRONMENT,
});

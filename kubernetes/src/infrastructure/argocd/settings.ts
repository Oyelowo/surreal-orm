import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const argocdProvider = getResourceProvider({
    outputDirectory: `infrastructure/argocd`,
    environment: ENVIRONMENT,
});

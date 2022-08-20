import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const argocdProvider = getResourceProvider({
    outputDirectory: `infrastructure/argocd`,
    environment: ENVIRONMENT,
});

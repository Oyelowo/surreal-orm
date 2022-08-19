import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

export const argocdProvider = getResourceProvider({
    outputDirectory: `infrastructure/argocd`,
    environment: ENVIRONMENT,
});

import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const certManagerProvider = getResourceProvider({
    outputDirectory: `infrastructure/cert-manager`,
    environment: ENVIRONMENT,
});

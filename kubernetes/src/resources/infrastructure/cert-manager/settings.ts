import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

export const certManagerProvider = getResourceProvider({
    outputDirectory: `infrastructure/cert-manager`,
    environment: ENVIRONMENT,
});

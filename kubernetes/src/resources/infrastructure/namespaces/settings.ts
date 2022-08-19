import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

export const namespacesNamesProvider = getResourceProvider({
    outputDirectory: `infrastructure/namespaces`,
    environment: ENVIRONMENT,
});

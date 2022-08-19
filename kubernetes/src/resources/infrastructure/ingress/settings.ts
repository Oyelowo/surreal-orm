import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

export const nginxIngressProvider = getResourceProvider({
    outputDirectory: `infrastructure/nginx-ingress`,
    environment: ENVIRONMENT,
});

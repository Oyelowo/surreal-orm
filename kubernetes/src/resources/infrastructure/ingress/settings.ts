import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const nginxIngressProvider = getResourceProvider({
    outputDirectory: `infrastructure/nginx-ingress`,
    environment: ENVIRONMENT,
});

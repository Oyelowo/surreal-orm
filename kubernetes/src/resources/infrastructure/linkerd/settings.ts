import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const linkerdProvider = getResourceProvider({
    outputDirectory: `infrastructure/linkerd`,
    environment: ENVIRONMENT,
});

export const linkerdVizProvider = getResourceProvider({
    outputDirectory: `infrastructure/linkerd-viz`,
    environment: ENVIRONMENT,
});


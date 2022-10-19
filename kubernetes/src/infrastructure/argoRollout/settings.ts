import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const argoRolloutProvider = getResourceProvider({
    outputDirectory: `infrastructure/argo-rollout`,
    environment: ENVIRONMENT,
});

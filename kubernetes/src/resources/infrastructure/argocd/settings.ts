import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const argocdProvider = getResourceProvider({
    resourcePath: `infrastructure/argocd`,
    environment: ENVIRONMENT,
});

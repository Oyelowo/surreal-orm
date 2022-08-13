import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const certManagerProvider = getResourceProvider({
    resourcePath: `infrastructure/cert-manager`,
    environment: ENVIRONMENT,
});

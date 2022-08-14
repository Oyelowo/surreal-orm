import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const certManagerProvider = getResourceProvider({
    outputDirectory: `infrastructure/cert-manager`,
    environment: ENVIRONMENT,
});

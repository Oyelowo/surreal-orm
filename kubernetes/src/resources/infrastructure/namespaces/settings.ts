import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const namespacesNamesProvider = getResourceProvider({
    outputDirectory: `infrastructure/namespaces`,
    environment: ENVIRONMENT,
});

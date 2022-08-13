import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const namespacesNamesProvider = getResourceProvider({
    resourceType: 'infrastructure',
    resourceName: 'namespaces',
    environment: ENVIRONMENT,
});

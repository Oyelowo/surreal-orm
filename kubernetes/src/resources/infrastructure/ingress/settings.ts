import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const nginxIngressProvider = getResourceProvider({
    resourceType: 'infrastructure',
    resourceName: 'nginx-ingress',
    environment: ENVIRONMENT,
});

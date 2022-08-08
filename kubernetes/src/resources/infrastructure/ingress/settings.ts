import { getResourceProvider } from '../../shared/manifestsDirectory.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const nginxIngressProvider = getResourceProvider('nginx-ingress', ENVIRONMENT);

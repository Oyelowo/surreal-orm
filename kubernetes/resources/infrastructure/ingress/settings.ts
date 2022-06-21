import { getResourceProvider } from '../../shared/manifestsDirectory';
import { getEnvironmentVariables } from '../../shared/validations';

const { ENVIRONMENT } = getEnvironmentVariables();
export const nginxIngressProvider = getResourceProvider('nginx-ingress', ENVIRONMENT);

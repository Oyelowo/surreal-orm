import { getResourceProvider } from '../../shared/manifestsDirectory';
import { getEnvironmentVariables } from '../../shared/validations';

const { ENVIRONMENT } = getEnvironmentVariables();
export const certManagerProvider = getResourceProvider('cert-manager', ENVIRONMENT);

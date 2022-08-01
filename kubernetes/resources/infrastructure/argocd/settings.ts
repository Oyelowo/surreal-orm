import { getResourceProvider } from '../../shared/manifestsDirectory.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const argocdProvider = getResourceProvider('argocd', ENVIRONMENT);

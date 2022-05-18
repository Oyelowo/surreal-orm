import { getResourceProvider } from '../../shared/manifestsDirectory';
import { getEnvironmentVariables } from '../../shared/validations';

const { ENVIRONMENT } = getEnvironmentVariables();
// export const { provider } = getResourceProperties("argocd", ENVIRONMENT);
export const argocdProvider = getResourceProvider('argocd', ENVIRONMENT);

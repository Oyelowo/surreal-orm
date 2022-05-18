import { getResourceProvider } from '../../shared/manifestsDirectory';
import { ResourceName } from '../../shared/types/own-types';
import { getEnvironmentVariables } from '../../shared/validations';

const { ENVIRONMENT } = getEnvironmentVariables();
export const sealedSecretsProvider = getResourceProvider('sealed-secrets', ENVIRONMENT);
export const resourceName: ResourceName = 'sealed-secrets';

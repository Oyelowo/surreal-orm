import { getResourceProvider } from '../../shared/manifestsDirectory';
import { ResourceName } from '../../shared/types/own-types';
import { getEnvironmentVariables } from '../../shared/validations';

const { ENVIRONMENT } = getEnvironmentVariables();
export const sealedSecretsResourceName: ResourceName = 'sealed-secrets';
export const sealedSecretsProvider = getResourceProvider(sealedSecretsResourceName, ENVIRONMENT);

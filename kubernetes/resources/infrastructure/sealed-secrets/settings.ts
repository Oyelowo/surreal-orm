import { getResourceProvider } from '../../shared/manifestsDirectory.js';
import { ResourceName } from '../../types/own-types.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const sealedSecretsResourceName: ResourceName = 'sealed-secrets';
export const sealedSecretsProvider = getResourceProvider(sealedSecretsResourceName, ENVIRONMENT);

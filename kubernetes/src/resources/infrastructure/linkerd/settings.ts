import { PlainSecretJsonConfig } from '../../../../scripts/utils/plainSecretJsonConfig.js';
import { getResourceProvider } from '../../shared/manifestsDirectory.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const linkerdProvider = getResourceProvider('linkerd', ENVIRONMENT);
export const linkerdVizProvider = getResourceProvider('linkerd-viz', ENVIRONMENT);
export const linkerdVizSecretsFromLocalConfigs = new PlainSecretJsonConfig('linkerd-viz', ENVIRONMENT).getSecrets();

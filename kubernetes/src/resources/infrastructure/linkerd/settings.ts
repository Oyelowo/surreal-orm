import { PlainSecretJsonConfig } from '../../../../scripts/utils/plainSecretJsonConfig.js';
import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const linkerdProvider = getResourceProvider({
    resourcePath: `infrastructure/linkerd`,
    environment: ENVIRONMENT,
});

export const linkerdVizProvider = getResourceProvider({
    resourcePath: `infrastructure/linkerd-viz`,
    environment: ENVIRONMENT,
});

export const linkerdVizSecretsFromLocalConfigs = new PlainSecretJsonConfig('linkerd-viz', ENVIRONMENT).getSecrets();

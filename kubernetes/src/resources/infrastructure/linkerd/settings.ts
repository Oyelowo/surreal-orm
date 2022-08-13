import { PlainSecretJsonConfig } from '../../../../scripts/utils/plainSecretJsonConfig.js';
import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const linkerdProvider = getResourceProvider({
    resourceType: 'infrastructure',
    resourceName: 'linkerd',
    environment: ENVIRONMENT,
});

export const linkerdVizProvider = getResourceProvider({
    resourceType: 'infrastructure',
    resourceName: 'linkerd-viz',
    environment: ENVIRONMENT,
});

export const linkerdVizSecretsFromLocalConfigs = new PlainSecretJsonConfig('linkerd-viz', ENVIRONMENT).getSecrets();

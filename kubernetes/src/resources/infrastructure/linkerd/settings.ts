import { PlainSecretJsonConfig } from '../../../../scripts/utils/plainSecretJsonConfig.js';
import { getResourceProvider } from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

export const linkerdProvider = getResourceProvider({
    outputDirectory: `infrastructure/linkerd`,
    environment: ENVIRONMENT,
});

export const linkerdVizProvider = getResourceProvider({
    outputDirectory: `infrastructure/linkerd-viz`,
    environment: ENVIRONMENT,
});

export const linkerdVizSecretsFromLocalConfigs = new PlainSecretJsonConfig('linkerd-viz', ENVIRONMENT).getSecrets();

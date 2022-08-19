import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

export const certManagerApplication = createArgocdApplication({
    environment: ENVIRONMENT,
    sourceAppDirectory: 'infrastructure/cert-manager',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    namespace: 'cert-manager',
});

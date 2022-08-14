import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();

export const certManagerApplication = createArgocdApplication({
    environment: ENVIRONMENT,
    sourceAppDirectory: 'infrastructure/cert-manager',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    namespace: 'cert-manager',
});

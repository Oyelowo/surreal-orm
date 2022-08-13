import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();

export const certManagerApplication = createArgocdApplication({
    resourceType: 'infrastructure',
    environment: ENVIRONMENT,
    sourceApplication: 'cert-manager',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: 'cert-manager',
});

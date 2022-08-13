import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const ingressControllerApplication = createArgocdApplication({
    resourceType: 'infrastructure',
    environment: ENVIRONMENT,
    sourceApplication: 'nginx-ingress',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: 'default',
});

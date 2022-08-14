import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
export const ingressControllerApplication = createArgocdApplication({
    sourceApplicationPath: 'infrastructure/nginx-ingress',
    outputPath: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'default',
});

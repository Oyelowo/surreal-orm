import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvVarsForKubeManifests } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const ingressControllerApplication = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/nginx-ingress',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'default',
});

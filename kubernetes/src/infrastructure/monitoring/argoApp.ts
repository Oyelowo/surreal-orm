import { createArgocdApplication } from '../argocd/createArgoApplication.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const argoEventApplication = createArgocdApplication({
    environment: ENVIRONMENT,
    sourceAppDirectory: 'infrastructure/argo-event',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    namespace: 'argo-event',
});

import { createArgocdApplication } from '../argocd/createArgoApplication.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const veleroApplication = createArgocdApplication({
    environment: ENVIRONMENT,
    sourceAppDirectory: 'infrastructure/velero',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    namespace: 'velero',
});

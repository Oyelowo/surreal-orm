import { createArgocdApplication } from '../argocd/createArgoApplication.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';
import { namespaces } from './util.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const namespacesArgoApps = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/namespaces',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: namespaces.default,
});

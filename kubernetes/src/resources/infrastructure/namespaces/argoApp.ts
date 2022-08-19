import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';
import { namespaces } from './util.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

export const namespacesArgoApps = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/namespaces',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: namespaces.default,
});

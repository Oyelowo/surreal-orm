import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';
import { namespaces } from './util.js';

const { ENVIRONMENT } = getEnvironmentVariables();

export const namespacesArgoApps = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/namespaces',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: namespaces.default,
});

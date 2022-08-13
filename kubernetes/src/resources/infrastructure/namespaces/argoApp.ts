import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';
import { namespaces } from './util.js';

const { ENVIRONMENT } = getEnvironmentVariables();

export const namespacesArgoApps = createArgocdApplication({
    resourceType: 'infrastructure',
    environment: ENVIRONMENT,
    sourceApplication: 'namespaces',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: namespaces.default,
});

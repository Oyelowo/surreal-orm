import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';
import { namespaces } from './util.js';

const { ENVIRONMENT } = getEnvironmentVariables();

export const namespacesArgoApps = createArgocdApplication({
    sourceApplicationName: 'namespaces',
    sourceApplicationPath: 'infrastructure/namespaces',
    outputPath: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: namespaces.default,
});

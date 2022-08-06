import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { namespaces } from './util.js';

export const namespacesArgoApps = createArgocdApplication({
    sourceApplication: 'namespaces',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: namespaces.default,
});

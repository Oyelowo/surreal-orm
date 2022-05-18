import { createArgocdApplication } from '../shared/createArgoApplication';
import { namespaceNames } from './util';

export const namespacesArgoApps = createArgocdApplication({
    resourceName: 'namespace-names',
    sourceResourceName: 'argocd-applications-children-infrastructure',
    namespace: namespaceNames.default,
});

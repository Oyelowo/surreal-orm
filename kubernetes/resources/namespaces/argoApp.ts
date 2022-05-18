import { createArgocdApplication } from '../shared/createArgoApplication';
import { namespaceNames } from './util';

export const namespacesArgoApps = createArgocdApplication({
    resourceName: 'argocd-applications-children-infrastructure',
    sourceResourceName: 'namespace-names',
    namespace: namespaceNames.default,
});

import { createArgocdApplication } from '../shared/createArgoApplication';
import { namespaceNames } from './util';

export const namespacesArgoApps = createArgocdApplication({
    sourceApplication: 'namespace-names',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: namespaceNames.default,
});

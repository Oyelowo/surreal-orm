import { createArgocdApplication } from '../../shared/createArgoApplication';
import { namespaces } from './util';

export const namespacesArgoApps = createArgocdApplication({
    sourceApplication: 'namespaces',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: namespaces.default,
});

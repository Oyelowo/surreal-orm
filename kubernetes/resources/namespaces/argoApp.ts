import { createArgocdApplication } from '../shared/createArgoApplication'
import { namespaceNames } from './util'

export const namespacesArgoApps = createArgocdApplication({
  sourceResourceName: 'argocd-applications-children-infrastructure',
  resourceName: 'namespace-names',
  namespace: namespaceNames.default
})

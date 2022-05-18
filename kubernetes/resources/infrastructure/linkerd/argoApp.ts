import { createArgocdApplication } from '../../shared/createArgoApplication'

// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
  sourceResourceName: 'argocd-applications-children-infrastructure',
  resourceName: 'linkerd',
  namespace: 'linkerd'
})

export const LinkerdVizApplication = createArgocdApplication({
  sourceResourceName: 'argocd-applications-children-infrastructure',
  resourceName: 'linkerd-viz',
  namespace: 'linkerd-viz'
})

import { createArgocdApplication } from '../../shared/createArgoApplication';

// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
    resourceName: 'argocd-applications-children-infrastructure',
    sourceResourceName: 'linkerd',
    namespace: 'linkerd',
});

export const LinkerdVizApplication = createArgocdApplication({
    resourceName: 'argocd-applications-children-infrastructure',
    sourceResourceName: 'linkerd-viz',
    namespace: 'linkerd-viz',
});

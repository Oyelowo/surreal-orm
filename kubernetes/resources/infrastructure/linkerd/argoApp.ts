import { createArgocdApplication } from '../../shared/createArgoApplication';

// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
    sourceApplication: 'linkerd',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: 'linkerd',
});

export const LinkerdVizApplication = createArgocdApplication({
    sourceApplication: 'linkerd-viz',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: 'linkerd-viz',
});

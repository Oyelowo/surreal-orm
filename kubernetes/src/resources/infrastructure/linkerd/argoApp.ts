import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

const { ENVIRONMENT } = getEnvironmentVariables();
// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
    sourceApplicationName: 'linkerd',
    sourceApplicationPath: 'infrastructure/linkerd',
    outputPath: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'linkerd',
});

export const LinkerdVizApplication = createArgocdApplication({
    sourceApplicationName: 'linkerd-viz',
    sourceApplicationPath: 'infrastructure/linkerd-viz',
    outputPath: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'linkerd-viz',
});

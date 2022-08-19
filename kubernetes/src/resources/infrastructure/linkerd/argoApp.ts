import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvVarsForKubeManifestGenerator } from '../../types/environmentVariables.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/linkerd',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'linkerd',
});

export const LinkerdVizApplication = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/linkerd-viz',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'linkerd-viz',
});

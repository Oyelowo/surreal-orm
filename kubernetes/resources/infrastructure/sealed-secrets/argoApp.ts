import { namespaceNames } from '../../namespaces';
import { createArgocdApplication } from '../../shared/createArgoApplication';

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
    resourceName: 'argocd-applications-children-infrastructure',
    sourceResourceName: 'sealed-secrets',
    namespace: namespaceNames.default,
});

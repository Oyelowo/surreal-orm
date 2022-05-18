import { namespaceNames } from '../../namespaces';
import { createArgocdApplication } from '../../shared/createArgoApplication';

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
    resourceName: 'sealed-secrets',
    sourceResourceName: 'argocd-applications-children-infrastructure',
    namespace: namespaceNames.default,
});

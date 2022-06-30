import { namespaces } from '../namespaces';
import { createArgocdApplication } from '../../shared/createArgoApplication';

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
    sourceApplication: 'sealed-secrets',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: namespaces.default,
});

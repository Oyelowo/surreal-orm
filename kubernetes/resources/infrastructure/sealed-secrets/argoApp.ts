import { namespaces } from '../namespaces/index.js';
import { createArgocdApplication } from '../../shared/createArgoApplication.js';

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
    sourceApplication: 'sealed-secrets',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: namespaces.default,
});

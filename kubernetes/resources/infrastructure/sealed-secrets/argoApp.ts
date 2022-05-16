import { namespaceNames } from "../../namespaces";
import { createArgocdChildrenApplication } from "../../shared/createArgoApplication";


// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdChildrenApplication({
  // resourceType: "infrastructure",
  argoResourceType: "argocd-applications-children-infrastructure",
  resourceName: "sealed-secrets",
  namespace: namespaceNames.default
});

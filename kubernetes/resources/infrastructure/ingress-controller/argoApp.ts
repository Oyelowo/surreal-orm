import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { createArgocdApplication, ingressControllerName, namespaceNames } from "../../shared";
import { ingressControllerDir } from "./ingressController";


// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const ingressControllerApplication = createArgocdApplication({
  metadata: {
    name: ingressControllerName,
    namespace: namespaceNames.default,
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(ingressControllerDir),
});

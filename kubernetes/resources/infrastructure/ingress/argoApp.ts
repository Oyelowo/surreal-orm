import { namespaceNames } from "../../namespaces/util";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { getRepoPathFromAbsolutePath, ingressControllerName } from "../../shared/manifestsDirectory";

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

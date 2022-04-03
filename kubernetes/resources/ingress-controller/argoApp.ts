import { createArgocdApplication, namespaceNames } from "../shared";
import { controllerName, ingressControllerDirName } from "./ingressController";

type Metadata = {
  name: string;
  namespace: string;
};

const metadataIngressController: Metadata = {
  name: controllerName,
  namespace: namespaceNames.default,
};

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
  metadata: { ...metadataIngressController },
  pathToAppManifests: ingressControllerDirName,
});

import { createArgocdApplication, namespaceNames } from "../shared";
import { ingressControllerName, ingressControllerDirName } from "./ingressController";

type Metadata = {
  name: string;
  namespace: string;
};

const metadataIngressController: Metadata = {
  name: ingressControllerName,
  namespace: namespaceNames.default,
};

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const ingressControllerApplication = createArgocdApplication({
  metadata: { ...metadataIngressController },
  pathToAppManifests: ingressControllerDirName,
});

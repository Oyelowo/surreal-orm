import { createArgocdApplication, namespaceNames } from "../shared";
import { controllerName, sealedSecretsControllerDirName } from "./sealedSecrets";

type Metadata = {
  name: string;
  namespace: string;
};

const metadataSealedSecretsController: Metadata = {
  name: controllerName,
  namespace: namespaceNames.default,
};

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
  metadata: { ...metadataSealedSecretsController },
  pathToAppManifests: sealedSecretsControllerDirName,
});

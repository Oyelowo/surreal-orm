import { createArgocdApplication, namespaceNames, sealedSecretsControllerName } from "../shared";
import { sealedSecretsControllerDirName } from "./sealedSecrets";

type Metadata = {
  name: string;
  namespace: string;
};

const metadataSealedSecretsController: Metadata = {
  name: sealedSecretsControllerName,
  namespace: namespaceNames.kubeSystem,
};

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
  metadata: { ...metadataSealedSecretsController },
  pathToAppManifests: sealedSecretsControllerDirName,
});

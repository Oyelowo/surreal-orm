import { namespaceNames } from "../../namespaces";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { getRepoPathFromAbsolutePath, sealedSecretsControllerName } from "./../../shared/manifestsDirectory";
import { sealedSecretsControllerDir } from "./sealedSecrets";

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
  metadata: {
    ...metadataSealedSecretsController,
    resourceType: "infrastructure" },
  pathToAppManifests: getRepoPathFromAbsolutePath(sealedSecretsControllerDir),
});

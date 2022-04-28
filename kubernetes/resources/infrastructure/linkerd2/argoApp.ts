import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { createArgocdApplication, namespaceNames, linkerd2Name } from "../../shared";
import { linkerd2Dir } from "./linkerd2";

type Metadata = {
  name: string;
  namespace: string;
};

const metadataSealedSecretsController: Metadata = {
  name: linkerd2Name,
  namespace: namespaceNames.linkerd,
};

// App that deploys sealedSecretsController resources themselves
/* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const sealedSecretsControllerApplication = createArgocdApplication({
  metadata: { ...metadataSealedSecretsController },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerd2Dir),
});

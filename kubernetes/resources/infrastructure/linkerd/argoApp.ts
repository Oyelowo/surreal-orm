import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { createArgocdApplication, namespaceNames, linkerd2Name } from "../../shared";
import { linkerd2Dir } from "./linkerd";

type Metadata = {
  name: string;
  namespace: string;
};

const metadataLinkerd2: Metadata = {
  name: linkerd2Name,
  namespace: namespaceNames.linkerd,
};

// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
  metadata: { ...metadataLinkerd2 },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerd2Dir),
});

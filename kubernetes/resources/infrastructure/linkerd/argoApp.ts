import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { createArgocdApplication, namespaceNames, linkerd2Name } from "../../shared";
import { linkerdDir } from "./linkerd";


// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
  metadata: {
    name: linkerd2Name,
    namespace: namespaceNames.linkerd,
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerdDir),
});

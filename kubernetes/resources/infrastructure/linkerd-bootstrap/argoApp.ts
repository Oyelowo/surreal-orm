import { linkerdBootstrapDir } from './provider';
import { namespaceNames } from "../../namespaces/util";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { getRepoPathFromAbsolutePath, linkerd2Name } from "../../shared/manifestsDirectory";


// App that deploys LinkerdBootStrap resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
  metadata: {
    name: linkerd2Name,
    namespace: namespaceNames.linkerd,
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerdBootstrapDir),
});

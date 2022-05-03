import { linkerdVizName } from './../../shared/manifestsDirectory';
import { linkerdVizDir } from './linkerdViz';
import { namespaceNames } from "../../namespaces/util";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { getRepoPathFromAbsolutePath, linkerd2Name } from "../../shared/manifestsDirectory";

import { linkerdDir } from "./linkerd";


// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
  metadata: {
    name: linkerd2Name,
    namespace: namespaceNames.linkerd,
    resourceType: "infrastructure"
    // argoApplicationName: "linkerd"
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerdDir),
});

export const LinkerdVizApplication = createArgocdApplication({
  metadata: {
    name: linkerdVizName,
    namespace: namespaceNames.linkerd,
    resourceType: "infrastructure"
    // argoApplicationName: "linkerd-viz"
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerdVizDir),
});



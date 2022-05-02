import { linkerdVizDir } from './linkerdViz';
import { linkerdBootstrapDir } from './provider';
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
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerdDir),
});


export const LinkerdBootstrapApplication = createArgocdApplication({
  metadata: {
    name: linkerd2Name,
    // We want the bootstrap stuff to be in cert manager namespace
    namespace: namespaceNames.certManager,
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerdBootstrapDir),
});

export const LinkerdVizApplication = createArgocdApplication({
  metadata: {
    name: linkerd2Name,
    namespace: namespaceNames.linkerd,
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(linkerdVizDir),
});



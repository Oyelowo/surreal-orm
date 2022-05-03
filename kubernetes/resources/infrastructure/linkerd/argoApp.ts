import { linkerdVizName } from './../../shared/manifestsDirectory';
import { linkerdVizDir } from './linkerdViz';
import { namespaceNames } from "../../namespaces/util";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { getRepoPathFromAbsolutePath, linkerd2Name } from "../../shared/manifestsDirectory";

import { linkerdDir } from "./linkerd";


// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
  resourceType: "infrastructure",
  resourceName: "linkerd",
  namespace: "linkerd"
});


export const LinkerdVizApplication = createArgocdApplication({
  resourceType: "infrastructure",
  resourceName: "linkerd-viz",
  namespace: "linkerd-viz"
});



import { linkerdVizName } from './../../shared/manifestsDirectory';
import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { linkerdVizDir } from "./linkerdViz";
import { createArgocdApplication } from '../../shared/createArgoApplication';
import { namespaceNames } from '../../namespaces/util';

// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const LinkerdVizApplication = createArgocdApplication({
    metadata: {
        name: linkerdVizName,
        namespace: namespaceNames.linkerdViz,
    },
    pathToAppManifests: getRepoPathFromAbsolutePath(linkerdVizDir),
});

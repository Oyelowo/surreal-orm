import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { createArgocdApplication, namespaceNames, linkerd2Name } from "../../shared";
import { linkerdVizDir } from "./linkerdViz";

type Metadata = {
    name: string;
    namespace: string;
};

const metadataLinkerdViz: Metadata = {
    name: linkerdVizDir,
    namespace: namespaceNames.linkerdViz,
};

// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
    metadata: { ...metadataLinkerdViz },
    pathToAppManifests: getRepoPathFromAbsolutePath(linkerdVizDir),
});

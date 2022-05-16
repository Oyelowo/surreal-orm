import { createArgocdChildrenApplication } from "../shared/createArgoApplication";
import { namespaceNames } from "./util";


export const namespacesArgoApps = createArgocdChildrenApplication({
    // resourceType: "namespaces",
    argoResourceType: "argocd-applications-children-infrastructure",
    resourceName: "namespace-names",
    namespace: namespaceNames.default
});


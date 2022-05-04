import { createArgocdApplication } from "../shared/createArgoApplication";
import { namespaceNames } from "./util";


export const namespacesArgoApps = createArgocdApplication({
    // resourceType: "namespaces",
    resourceName: "namespace-names",
    namespace: namespaceNames.default
});


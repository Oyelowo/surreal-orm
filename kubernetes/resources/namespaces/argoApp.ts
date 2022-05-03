import { getEnvironmentVariables } from './../shared/validations';
// Use either, Both work, but I'm using the offical one for now as it is stable and has support for notification. 

import { createArgocdApplication } from "../shared/createArgoApplication";
import { getNamespacesNamesDir, getRepoPathFromAbsolutePath } from "../shared/manifestsDirectory";
import { namespaceNames } from "./util";


// This is the argo application for update namespace resource manifests
// 
export const namespacesArgoApps = createArgocdApplication({
    resourceType: "namespaces",
    resourceName: "namespace-names",
    namespace: namespaceNames.default
});


import { namespaceNames } from "../../namespaces/util";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { certManagerControllerName, getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";




export const certManagerApplication = createArgocdApplication({
  resourceType: "infrastructure",
  resourceName: "cert-manager",
  namespace: "cert-manager"
});

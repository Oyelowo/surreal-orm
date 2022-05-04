import { namespaceNames } from "../../namespaces/util";
import { createArgocdApplication } from "../../shared/createArgoApplication";


export const certManagerApplication = createArgocdApplication({
  // resourceType: "infrastructure",
  resourceName: "cert-manager",
  namespace: "cert-manager"
});

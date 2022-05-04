import { createArgocdChildrenApplication } from "../../shared/createArgoApplication";


export const certManagerApplication = createArgocdChildrenApplication({
  // resourceType: "infrastructure",
  resourceName: "cert-manager",
  namespace: "cert-manager"
});

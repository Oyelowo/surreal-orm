import { createArgocdChildrenApplication } from "../../shared/createArgoApplication";


export const certManagerApplication = createArgocdChildrenApplication({
  // resourceType: "infrastructure",
  argoResourceType: "argocd-applications-children-infrastructure",
  resourceName: "cert-manager",
  namespace: "cert-manager",
});

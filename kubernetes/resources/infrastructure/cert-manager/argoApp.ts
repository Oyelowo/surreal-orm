import { createArgocdApplication } from "../../shared/createArgoApplication";


export const certManagerApplication = createArgocdApplication({
  sourceResourceName: "argocd-applications-children-infrastructure",
  resourceName: "cert-manager",
  namespace: "cert-manager",
});

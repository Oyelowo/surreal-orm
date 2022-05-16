
import { createArgocdChildrenApplication } from "../../shared/createArgoApplication"

export const ingressControllerApplication = createArgocdChildrenApplication({
  argoResourceType: "argocd-applications-children-infrastructure",
  resourceName: "nginx-ingress",
  namespace: "default"
});

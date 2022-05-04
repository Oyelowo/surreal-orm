
import { createArgocdChildrenApplication } from "../../shared/createArgoApplication"

export const ingressControllerApplication = createArgocdChildrenApplication({
  // resourceType: "infrastructure",
  resourceName: "nginx-ingress",
  namespace: "default"
});

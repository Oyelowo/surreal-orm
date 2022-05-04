
import { createArgocdApplication } from "../../shared/createArgoApplication"

export const ingressControllerApplication = createArgocdApplication({
  // resourceType: "infrastructure",
  resourceName: "nginx-ingress",
  namespace: "default"
});

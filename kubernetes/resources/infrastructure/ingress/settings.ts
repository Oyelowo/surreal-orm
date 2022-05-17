import { getResourceProvider } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
// export const { provider, resourceName } = getResourceProperties("nginx-ingress", ENVIRONMENT);
export const nginxIngressProvider = getResourceProvider(
  "nginx-ingress",
  ENVIRONMENT
);

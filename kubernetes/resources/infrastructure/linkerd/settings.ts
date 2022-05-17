import { getResourceProvider } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
export const linkerdProvider = getResourceProvider("linkerd", ENVIRONMENT);
export const linkerdVizProvider = getResourceProvider(
  "linkerd-viz",
  ENVIRONMENT
);

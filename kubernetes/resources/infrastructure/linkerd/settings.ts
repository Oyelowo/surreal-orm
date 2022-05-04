import { getResourceProperties } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
export const linkerdProperties = getResourceProperties("linkerd", ENVIRONMENT);
export const linkerdVizProperties = getResourceProperties("linkerd-viz", ENVIRONMENT);

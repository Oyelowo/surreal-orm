import { getResourceProperties } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
// export const { provider } = getResourceProperties("cert-manager", ENVIRONMENT);
export const certManagerProperties = getResourceProperties("cert-manager", ENVIRONMENT);

import { getResourceProvider } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
// export const { provider } = getResourceProperties("cert-manager", ENVIRONMENT);
export const certManagerProvider = getResourceProvider("cert-manager", ENVIRONMENT);

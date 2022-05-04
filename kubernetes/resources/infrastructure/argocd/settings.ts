import { getResourceProperties } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
// export const { provider } = getResourceProperties("argocd", ENVIRONMENT);
export const argocdProperties = getResourceProperties("argocd", ENVIRONMENT);

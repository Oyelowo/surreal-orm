import { getResourceProperties } from "../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../shared/validations";


const { ENVIRONMENT } = getEnvironmentVariables();
export const namespacesNamesProperties = getResourceProperties("namespace-names", ENVIRONMENT);

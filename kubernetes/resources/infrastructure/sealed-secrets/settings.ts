import { ResourceName } from './../../shared/manifestsDirectory';
import { getResourceProvider } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
export const sealedSecretsProvider = getResourceProvider("sealed-secrets", ENVIRONMENT);
export const resourceName: ResourceName = "sealed-secrets"
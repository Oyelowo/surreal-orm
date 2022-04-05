import { AppName } from "./../resources/shared/types/own-types";
import { SECRET_LOCAL } from "./secrets-unsealed/local";
import { SECRET_DEVELOPMENT } from "./secrets-unsealed/development";
import { SECRET_STAGING } from "./secrets-unsealed/staging";
import { SECRET_PRODUCTION } from "./secrets-unsealed/production";
import { Environment } from "../resources/shared/types/own-types";
import { Secrets } from "./setupSecrets";
import { getEnvironmentVariables } from "../resources/shared/validations";

const secretRecord: Record<Environment, Secrets> = {
  production: SECRET_PRODUCTION,
  staging: SECRET_STAGING,
  development: SECRET_DEVELOPMENT,
  local: SECRET_LOCAL,
};

export function getSecretForApp<App extends AppName>(appName: App): typeof secretRecord[Environment][App] {
  return secretRecord[getEnvironmentVariables().ENVIRONMENT][appName];
}

import { AppName } from "./../resources/shared/types/own-types";
import { SECRET_LOCAL } from "./secrets-unsealed/local";
import { SECRET_DEVELOPMENT } from "./secrets-unsealed/development";
import { SECRET_STAGING } from "./secrets-unsealed/staging";
import { SECRET_PRODUCTION } from "./secrets-unsealed/production";
import { Environment } from "../resources/shared/types/own-types";
import { Secrets } from "./setupSecrets";
import { environmentVariables } from "../resources/shared/validations";

const secretRecord: Record<Environment, Secrets> = {
  production: SECRET_PRODUCTION,
  staging: SECRET_STAGING,
  development: SECRET_DEVELOPMENT,
  local: SECRET_LOCAL,
};

export function getEnvironmentSecretForApp<App extends AppName>(appName: App) {
  return secretRecord[environmentVariables.ENVIRONMENT][appName];
}

import { AppName } from "../../resources/shared/types/own-types";
import { Environment } from "../../resources/shared/types/own-types";
import { Secrets } from "./setupSecrets";
import { getEnvironmentVariables } from "../../resources/shared/validations";
import { SECRET_DEVELOPMENT } from "../../.secrets/development";
import { SECRET_LOCAL } from "../../.secrets/local";
import { SECRET_PRODUCTION } from "../../.secrets/production";
import { SECRET_STAGING } from "../../.secrets/staging";

const secretRecord: Record<Environment, Secrets> = {
  production: SECRET_PRODUCTION,
  staging: SECRET_STAGING,
  development: SECRET_DEVELOPMENT,
  local: SECRET_LOCAL,
};

export function getSecretsForApp<App extends AppName>(appName: App): typeof secretRecord[Environment][App] {
  return secretRecord[getEnvironmentVariables().ENVIRONMENT][appName];
}

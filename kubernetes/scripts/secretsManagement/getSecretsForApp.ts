import { Environment, ResourceName } from "./../../resources/shared/types/own-types";
import { ServiceName } from "../../resources/shared/types/own-types";
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

type AppSecrets<App extends ResourceName> = typeof secretRecord[Environment][App];

export function getSecretsForApp<App extends ResourceName>(
  appName: App,
  environment: Environment
): AppSecrets<App> {
  // return secretRecord[getEnvironmentVariables().ENVIRONMENT][appName];
  return secretRecord[environment][appName];
}

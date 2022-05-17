import {
  Environment,
  ResourceName,
} from "./../../resources/shared/types/own-types";
import { SECRET_DEVELOPMENT } from "../../.secrets/development";
import { SECRET_LOCAL } from "../../.secrets/local";
import { SECRET_PRODUCTION } from "../../.secrets/production";
import { SECRET_STAGING } from "../../.secrets/staging";
import { Secrets } from "./setupSecrets";

export const secretRecord: Record<Environment, Secrets> = {
  production: SECRET_PRODUCTION,
  staging: SECRET_STAGING,
  development: SECRET_DEVELOPMENT,
  local: SECRET_LOCAL,
};

type AppSecrets<App extends ResourceName> =
  typeof secretRecord[Environment][App];

export function getSecretsForResource<Resource extends ResourceName>(
  resourceName: Resource,
  environment: Environment
): AppSecrets<Resource> {
  return secretRecord[environment][resourceName];
}

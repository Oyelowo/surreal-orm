// NOTE: This file is intended to be called via shell command
// line because the imported secrets may not exist during the course of
// running the code
import { ENVIRONMENTS_ALL } from "../utils/sealedSecrets";
import { Environment } from "../../resources/shared/types/own-types";
import sh from "shelljs";
import { secretsSample } from "./secretsSample";

import { SECRET_LOCAL } from "../../.secrets/local";
import { SECRET_STAGING } from "../../.secrets/staging";
import { SECRET_DEVELOPMENT } from "../../.secrets/development";
import { SECRET_PRODUCTION } from "../../.secrets/production";

import R from "ramda";
import {
  getPlainSecretInputFilePath,
  Secrets,
  getPlainSecretsContent,
} from "./setupSecrets";

function getMerged(environment: Environment) {
  const secretsByEnv: Record<Environment, Secrets> = {
    local: SECRET_LOCAL,
    development: SECRET_DEVELOPMENT,
    staging: SECRET_STAGING,
    production: SECRET_PRODUCTION,
  };

  const existingContent = secretsByEnv[environment] ?? {};
  const secrets = R.mergeDeepLeft(existingContent, secretsSample);
  const filePath = getPlainSecretInputFilePath(environment);
  const content = getPlainSecretsContent({ environment, secrets });

  sh.exec(`echo ${content} > ${filePath}`);
}

function main() {
  ENVIRONMENTS_ALL.forEach(getMerged);
}
main()

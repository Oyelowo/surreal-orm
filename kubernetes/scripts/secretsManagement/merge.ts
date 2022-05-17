import { ENVIRONMENTS_ALL } from "./../utils/sealedSecrets";
import { Environment } from "./../../resources/shared/types/own-types";
import sh from "shelljs";
import { secretsSample } from "./secretsSample";

import { SECRET_LOCAL } from "./../../.secrets/local";
import { SECRET_STAGING } from "./../../.secrets/staging";
import { SECRET_DEVELOPMENT } from "./../../.secrets/development";
import { SECRET_PRODUCTION } from "./../../.secrets/production";

import R from "ramda";
import {
  getPlainSecretInputFilePath,
  Secrets,
  getContent,
} from "./setupSecrets";

export function getMerged(environment: Environment) {
  const kk: Record<Environment, Secrets> = {
    local: SECRET_LOCAL,
    development: SECRET_DEVELOPMENT,
    staging: SECRET_STAGING,
    production: SECRET_PRODUCTION,
  };

  const existingContent = kk[environment] ?? {};
  const secrets = R.mergeDeepLeft(existingContent, secretsSample);
  const filePath = getPlainSecretInputFilePath(environment);
  const content = getContent({ environment, secrets });

  sh.exec(`echo ${content} > ${filePath}`);
  // return content
  // sh.touch(getFilePath(environment))
}

ENVIRONMENTS_ALL.forEach(getMerged);
// main()

// import("../../.secrets/staging")

// getMerged("staging")

// function kk(environment: Environment) {
//   sh.touch("file.ts");

//   const ss = `SECRET_${environment.toLocaleUpperCase()}`
//   const k = `
//   import sh from "shelljs";
//   import R from "ramda";
//     import { ${ss}} } from './../../.secrets/${environment}';
//     import { secretsSample } from "./secretsSample";

//    const merged = R.mergeDeepLeft(${ss}, secretsSample)}

//    sh.exec()

//   `
// }

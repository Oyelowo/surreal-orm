// NOTE: This file is intended to be called via shell command
// line because the imported secrets may not exist during the course of
// running the code
import { ENVIRONMENTS_ALL } from "../utils/sealedSecrets";
import { Environment } from "../../resources/shared/types/own-types";
import sh from "shelljs";
import { secretsSample } from "./secretsSample";
import R from "ramda";
import {
  getPlainSecretInputFilePath,
  getPlainSecretsContent,
} from "./setupSecrets";
import { secretRecord } from "./getSecretsForApp";

function mergeWithExistingSecrets(environment: Environment) {
  const existingContent = secretRecord[environment] ?? {};
  const secrets = R.mergeDeepLeft(existingContent, secretsSample);
  const filePath = getPlainSecretInputFilePath(environment);
  const content = getPlainSecretsContent({ environment, secrets });

  sh.exec(`echo ${content} > ${filePath}`);
}

function main() {
  ENVIRONMENTS_ALL.forEach(mergeWithExistingSecrets);
}
main();

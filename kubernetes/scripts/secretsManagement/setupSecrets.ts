/* 
TODO: ADD INSTRUCTION HERE
 */
import sh from "shelljs";
import path from "path";
import { Environment } from "../../resources/shared/types/own-types";
import { getPlainSecretsConfigFilesBaseDir } from "./../../resources/shared/manifestsDirectory";
import { secretsSample } from "./secretsSample";

const ENVIRONMENTS: Environment[] = [
  "local",
  "development",
  "staging",
  "production",
];
const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();

type PlainInputSecretsFilePath =
  `${typeof PLAIN_SECRETS_CONFIGS_DIR}/${Environment}.ts`;

export function setupPlainSecretTSFiles() {
  sh.mkdir(PLAIN_SECRETS_CONFIGS_DIR);
  ENVIRONMENTS.forEach(createSecretsConfigFile);
}

export function clearPlainInputTsSecretFilesContents() {
  const removeSecret = (env: Environment) =>
    sh.rm("-rf", getPlainSecretInputFilePath(env));
  ENVIRONMENTS.forEach(removeSecret);

  setupPlainSecretTSFiles();
}

export function getPlainSecretInputFilePath(
  environment: Environment
): PlainInputSecretsFilePath {
  return `${PLAIN_SECRETS_CONFIGS_DIR}/${environment}.ts`;
}

export type Secrets = typeof secretsSample;

async function createSecretsConfigFile(environment: Environment) {
  const filePath = getPlainSecretInputFilePath(environment);
  const content = getContent({ environment, secrets: secretsSample });

  sh.mkdir(path.dirname(filePath));
  sh.touch(filePath);
  sh.exec(`echo "$(echo '// @ts-nocheck'; cat ${filePath})" > ${filePath}`);
  // TODO: This check can be improved to check the serialized content against the sample
  const secretsExists = !!sh.cat(filePath)?.stdout?.trim();

  if (secretsExists) {
    try {
      sh.exec("npx ts-node ./scripts/secretsManagement/merge.ts");
    } catch (error) {}
    return;
  }

  sh.exec(`echo ${content} > ${filePath}`);
}

export function getContent({
  environment,
  secrets,
}: {
  environment: Environment;
  secrets: Secrets;
}) {
  const thisFileRelativeDir = __dirname.split("/").slice(-2).join("/");
  const thisFileName = path.basename(__filename).slice(0, -3);
  const SECRETS_TYPE = "Secrets" as const; // This should be same as the secrets type above

  return JSON.stringify(`
    import {${SECRETS_TYPE}} from "../${thisFileRelativeDir}/${thisFileName}";
    
     export const SECRET_${environment.toUpperCase()}: ${SECRETS_TYPE} = ${JSON.stringify(
    secrets
  )};
    `);
}

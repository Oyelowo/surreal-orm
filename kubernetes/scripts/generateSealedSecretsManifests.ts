import { Environment } from "./../resources/shared/types/own-types";
import path from "path";
import sh from "shelljs";
import c from "chalk";
import { clearUnsealedInputTsSecretFilesContents } from "../secretsManagement/setupSecrets";

import { globAsync } from "./bootstrap";

export const MANIFESTS_DIR = path.join(__dirname, "..", "manifests");

/* 
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
type GenSealedSecretsProps = {
  environment: Environment;
  keepSecretOutputs: boolean;
  keepSecretInputs: boolean;
  generateSealedSecrets: boolean;
};

export async function generateSealedSecretsManifests({
  environment,
  keepSecretInputs: keepSecretInputs,
  keepSecretOutputs,
  generateSealedSecrets,
}: GenSealedSecretsProps) {
  const contextDir = path.join(__dirname, "..", "manifests", "generated", environment);
  const unsealedSecretsFilePathsForEnv = getFilePathsThatMatch({
    contextDir,
    pattern: "secret-*ml",
  });

  for (const unsealedSecretPath of unsealedSecretsFilePathsForEnv) {
    if (generateSealedSecrets) {
      const unsealedSecretDir = path.dirname(unsealedSecretPath);
      // Secrets are prefixed with "secret-"
      const unsealedSecretFileName = path.basename(unsealedSecretPath);
      const sealedSecretFileName = `sealed-${unsealedSecretFileName}`;
      const sealedSecretFilePath = path.join(unsealedSecretDir, sealedSecretFileName);

      sh.echo(
        c.blueBright(
          `Generating sealed secret for ${unsealedSecretFileName} at ${unsealedSecretDir}`
        )
      );

      const kubeSeal = sh.exec(`kubeseal <${unsealedSecretPath} -o yaml >${sealedSecretFilePath}`, {
        silent: true,
      });

      sh.echo(c.greenBright(kubeSeal.stdout));
      if (kubeSeal.stderr) {
        sh.echo(`Error sealing secrets: ${c.redBright(kubeSeal.stderr)}`);
        sh.exit(1);
        return;
      }

      sh.echo(c.greenBright("Successfully generated sealed secret at", unsealedSecretPath));
    }

    sh.echo(c.blueBright(`Removing unsealed plain secret manifest ${unsealedSecretPath}`));

    // Delete unsealed plain secret if specified
    if (!keepSecretOutputs) {
      sh.rm("-rf", unsealedSecretPath);
    }

    if (!keepSecretInputs) {
      clearUnsealedInputTsSecretFilesContents();
    }
  }
}
// const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = sh.exec(`find ${contextDir} -name "secret-*ml"`, {
export function getFilePathsThatMatch({
  contextDir,
  pattern,
}: {
  contextDir: string;
  pattern: string;
}) {
  const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = sh.exec(`find ${contextDir} -name "${pattern}"`, {
    silent: true,
  });
  const unsealedSecretsFilePathsForEnv = UNSEALED_SECRETS_MANIFESTS_FOR_ENV.stdout
    .trim()
    .split("\n")
    .map((s) => s.trim());
  return unsealedSecretsFilePathsForEnv;
}

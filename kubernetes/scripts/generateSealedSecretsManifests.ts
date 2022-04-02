import { Environment } from "./../resources/shared/types/own-types";
import path from "path";
import sh from "shelljs";
import c from "chalk";
import { clearUnsealedInputTsSecretFilesContents } from "../secretsManagement/setupSecrets";

import { globAsync } from "./script";

export const MANIFESTS_DIR = path.join(__dirname, "..", "manifests");
export const SEALED_SECRETS_BASE_DIR = path.join(MANIFESTS_DIR, "sealed-secrets");

export const getSecretDirForEnv = (environment: Environment) =>
  path.join(SEALED_SECRETS_BASE_DIR, environment);
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
  const SEALED_SECRETS_DIR_FOR_ENV = getSecretDirForEnv(environment);
  const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = path.join(
    MANIFESTS_DIR,
    "generated",
    environment,
    "/**/**/**secret-*ml"
  );

  const unsealedSecretsFilePathsForEnv = await globAsync(UNSEALED_SECRETS_MANIFESTS_FOR_ENV, {
    dot: true,
  });

  unsealedSecretsFilePathsForEnv.forEach((unsealedSecretPath) => {
    if (generateSealedSecrets) {
      sh.echo(c.blueBright("Generating sealed secret from", unsealedSecretPath));

      const secretName = path.basename(unsealedSecretPath);
      const sealedSecretFilePath = path.join(SEALED_SECRETS_DIR_FOR_ENV, secretName);

      sh.echo(c.blueBright(`Create ${SEALED_SECRETS_DIR_FOR_ENV} if it does not exists`));
      sh.mkdir("-p", SEALED_SECRETS_DIR_FOR_ENV);

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

    if (!keepSecretOutputs) {
      sh.rm("-rf", unsealedSecretPath);
    }

    if (!keepSecretInputs) {
      clearUnsealedInputTsSecretFilesContents();
    }
  });
}

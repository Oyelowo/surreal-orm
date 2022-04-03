import { ImageTags } from "./../resources/shared/validations";
import sh from "shelljs";
import { Environment } from "../resources/shared/types/own-types";
import { manifestsDirForEnv, ENVIRONMENT, ARGV } from "./bootstrap";
import path from "path";

/*
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/
interface GenerateManifestsProps extends GenSealedSecretsProps {
  imageTags: ImageTags;
}

export async function generateManifests({
  environment,
  imageTags,
  regenerateSealedSecrets: generateSealedSecrets,
  keepSecretInputs,
  keepSecretOutputs,
}: GenerateManifestsProps) {
  sh.exec("npm i");
  sh.rm("-rf", "./login");
  sh.mkdir("./login");

  // We are temporarily renaming existing files before generating new ones,
  // So that we can retain the sealed secrets which is not managed by pulumi
  // but by our cli tool/kubeseal
  const temporaryDirSuffixString = "-temporaryxxxxxxxxxxx";
  const temporaryDir = `${manifestsDirForEnv}${temporaryDirSuffixString}`;
  // Rename old dir
  sh.mv(manifestsDirForEnv, temporaryDir);

  sh.exec("pulumi login file://login");
  sh.exec("export PULUMI_CONFIG_PASSPHRASE='' && pulumi stack init --stack dev");

  // image tag. All apps share same tag for now
  // Pulumi needs some environment variables set for generating deployments with image tag
  /* `export ${IMAGE_TAG_REACT_WEB}="${ARGV.t}" && \ `
     `export ${IMAGE_TAG_REACT_WEB}="${ARGV.t}" && \ `
     */
  const imageEnvVarSetterForPulumi = Object.entries(imageTags)
    .map(([k, v]) => `export ${k}=${v}`)
    .join(" ");

  sh.exec(
    `
      ${imageEnvVarSetterForPulumi} 
      export ${ENVIRONMENT}=${environment}  
      export PULUMI_CONFIG_PASSPHRASE="" 
      pulumi update --yes --skip-preview --stack dev
      `
  );

  // Write them back
  const sealedSecretsFilePathsForEnvTemporary = getFilePathsThatMatch({
    contextDir: temporaryDir,
    pattern: "sealed-secret-*ml",
  });

  for (const sealedSecretFilePath of sealedSecretsFilePathsForEnvTemporary) {
    const tempDirname = path.dirname(sealedSecretFilePath);
    // remove the suffix to form original directory name
    const originalDir = tempDirname.split(temporaryDirSuffixString).join("");
    const fileName = path.basename(sealedSecretFilePath);
    const constructFilePath = (dirname: string) => path.join(dirname, fileName);
    sh.cp(constructFilePath(tempDirname), constructFilePath(originalDir));
  }

  sh.rm("-rf", temporaryDir);

  regenerateSealedSecretsManifests({
    keepSecretInputs,
    keepSecretOutputs,
    regenerateSealedSecrets: generateSealedSecrets,
    environment,
  });
}

import c from "chalk";
import { clearUnsealedInputTsSecretFilesContents } from "../secretsManagement/setupSecrets";

export const MANIFESTS_DIR = path.join(__dirname, "..", "manifests");

/* 
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
interface GenSealedSecretsProps {
  environment: Environment;
  keepSecretOutputs: boolean;
  keepSecretInputs: boolean;
  regenerateSealedSecrets: boolean;
}

export async function regenerateSealedSecretsManifests({
  environment,
  keepSecretInputs,
  keepSecretOutputs,
  regenerateSealedSecrets,
}: GenSealedSecretsProps) {
  const contextDir = path.join(__dirname, "..", "manifests", "generated", environment);
  const unsealedSecretsFilePathsForEnv = getFilePathsThatMatch({
    contextDir,
    pattern: "secret-*ml",
  });

  for (const unsealedSecretPath of unsealedSecretsFilePathsForEnv) {
    if (regenerateSealedSecrets) {
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

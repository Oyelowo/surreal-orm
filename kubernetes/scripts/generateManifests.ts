import { getGeneratedEnvManifestsDir } from "./../resources/shared/manifestsDirectory";
import { ImageTags } from "./../resources/shared/validations";
import sh from "shelljs";
import { Environment } from "../resources/shared/types/own-types";
import { ENVIRONMENT } from "./bootstrap";
import p from "path";
import c from "chalk";
import { clearUnsealedInputTsSecretFilesContents } from "./secretsManagement/setupSecrets";

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

  sh.exec("pulumi login file://login");
  sh.echo(c.blueBright(`First Delete old resources for" ${environment}`));

  const manifestsDirForEnv = getGeneratedEnvManifestsDir(environment);

  sh.exec(`find ${manifestsDirForEnv} -type d -name "1-manifest" -prune -exec rm -rf {} \;`);
  sh.exec(`find ${manifestsDirForEnv} -type d -name "0-crd" -prune -exec rm -rf {} \;`);

  sh.exec("export PULUMI_CONFIG_PASSPHRASE='' && pulumi stack init --stack dev");

  // Pulumi needs some environment variables set for generating deployments with image tag
  /* `export ${IMAGE_TAG_REACT_WEB}=tag-web export ${IMAGE_TAG_GRAPHQL_MONGO}=tag-mongo`
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

  regenerateSealedSecretsManifests({
    keepSecretInputs,
    keepSecretOutputs,
    regenerateSealedSecrets: generateSealedSecrets,
    environment,
  });
}

// TODO: Use from shared helper
export const MANIFESTS_DIR = p.join(__dirname, "..", "manifests");

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
  const contextDir = p.join(__dirname, "..", "manifests", "generated", environment);
  const unsealedSecretsFilePathsForEnv = getFilePathsThatMatch({
    contextDir,
    pattern: "secret-*ml",
  });

  for (const unsealedSecretManifestPath of unsealedSecretsFilePathsForEnv) {
    const appManifestsDir = p.dirname(unsealedSecretManifestPath);
    if (regenerateSealedSecrets) {
      // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
      const appBaseDir = p.join(appManifestsDir, "..");
      const unsealedSecretResourceFileName = p.basename(unsealedSecretManifestPath);
      const sealedSecretDirForApp = p.join(appBaseDir, "sealed-secrets");
      const sealedSecretFilePath = p.join(sealedSecretDirForApp, `sealed-${unsealedSecretResourceFileName}`);

      sh.mkdir(sealedSecretDirForApp);

      // TODO: Check the content of the file to confirm if it is actually a secret object
      sh.echo(
        c.blueBright(`Generating sealed secret ${unsealedSecretManifestPath} \n to \n ${sealedSecretFilePath}`)
      );

      // TODO: Should I delete old sealed secrets before creating new ones?
      const kubeSeal = sh.exec(`kubeseal < ${unsealedSecretManifestPath} -o yaml >${sealedSecretFilePath}`, {
        silent: true,
      });

      sh.echo(c.greenBright(kubeSeal.stdout));
      if (kubeSeal.stderr) {
        sh.echo(`Error sealing secrets: ${c.redBright(kubeSeal.stderr)}`);
        sh.exit(1);
        return;
      }

      sh.echo(c.greenBright("Successfully generated sealed secret at", unsealedSecretManifestPath));
    }

    sh.echo(c.blueBright(`Removing unsealed plain secret manifest ${unsealedSecretManifestPath}`));

    // Delete unsealed plain secret if specified
    if (!keepSecretOutputs) {
      sh.rm("-rf", unsealedSecretManifestPath);
    }

    if (!keepSecretInputs) {
      clearUnsealedInputTsSecretFilesContents();
    }
  }
}

export function getFilePathsThatMatch({ contextDir, pattern }: { contextDir: string; pattern: string }) {
  const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = sh.exec(`find ${contextDir} -name "${pattern}"`, {
    silent: true,
  });
  const unsealedSecretsFilePathsForEnv = UNSEALED_SECRETS_MANIFESTS_FOR_ENV.stdout
    .trim()
    .split("\n")
    .map((s) => s.trim());
  return unsealedSecretsFilePathsForEnv;
}

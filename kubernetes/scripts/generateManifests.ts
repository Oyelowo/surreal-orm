import { getGeneratedEnvManifestsDir, sealedSecretsControllerName } from "./../resources/shared/manifestsDirectory";
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
interface GenerateManifestsProps extends Pick<GenSealedSecretsProps, "environment"> {
  imageTags: ImageTags;
}

export async function generateManifests({ environment, imageTags }: GenerateManifestsProps) {
  const manifestsDirForEnv = getGeneratedEnvManifestsDir(environment);
  sh.exec("npm i");
  sh.rm("-rf", "./login");
  sh.mkdir("./login");

  sh.exec("pulumi login file://login");

  sh.echo(c.blueBright(`First Delete old resources for" ${environment} at ${manifestsDirForEnv}`));

  const getManifestsWithinDirName = (dirName: "1-manifest" | "0-crd") => sh.exec(`find ${manifestsDirForEnv} -type d -name "${dirName}"`, { silent: true }).stdout.split("\n");
  const manifestsNonCrds = getManifestsWithinDirName("1-manifest");
  const manifestsCrds = getManifestsWithinDirName("0-crd");
  manifestsNonCrds.concat(manifestsCrds).forEach(f => sh.rm("-rf", f.trim()));

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
      pulumi up --yes --skip-preview --stack dev
      `
  );

  sh.rm("-rf", "./login");
}

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
  // const contextDir = p.join(__dirname, "..", "manifests", "generated", environment);
  const contextDir = getGeneratedEnvManifestsDir(environment);
  const unsealedSecretsFilePathsForEnv = getFilePathsThatMatch({
    contextDir,
    pattern: "secret-*ml",
  });

  for (const unsealedSecretManifestPath of unsealedSecretsFilePathsForEnv) {
    const appManifestsDir = p.dirname(unsealedSecretManifestPath);

    if (regenerateSealedSecrets) {
      // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
      // and we want as basedir: kubernetes/manifests/generated/production/applications/graphql-mongo
      const appBaseDir = p.join(appManifestsDir, "..");
      const unsealedSecretManifestFileName = p.basename(unsealedSecretManifestPath);
      const sealedSecretDir = p.join(appBaseDir, "sealed-secrets");
      const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-${unsealedSecretManifestFileName}`);

      sh.mkdir(sealedSecretDir);

      // TODO: Check the content of the file to confirm if it is actually a secret object
      sh.echo(
        c.blueBright(
          `Generating sealed secret ${unsealedSecretManifestPath} \n to \n ${sealedSecretFilePath}`
        )
      );

      // TODO: Should I delete old sealed secrets before creating new ones?
      const kubeSeal = sh.exec(`kubeseal --controller-name ${sealedSecretsControllerName} < ${unsealedSecretManifestPath} -o yaml >${sealedSecretFilePath}`, {
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

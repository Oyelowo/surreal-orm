import {
  getGeneratedEnvManifestsDir,
  ResourceName,
} from "./../resources/shared/manifestsDirectory";
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
interface GenerateManifestsProps
  extends Pick<GenSealedSecretsProps, "environment"> {
  imageTags: ImageTags;
}

export async function generateManifests({
  environment,
  imageTags,
}: GenerateManifestsProps) {
  const manifestsDirForEnv = getGeneratedEnvManifestsDir(environment);
  sh.exec("npm i");
  sh.rm("-rf", "./login");
  sh.mkdir("./login");

  sh.exec("pulumi login file://login");

  sh.echo(
    c.blueBright(
      `First Delete old resources for" ${environment} at ${manifestsDirForEnv}`
    )
  );

  const getManifestsWithinDirName = (dirName: "1-manifest" | "0-crd") =>
    sh
      .exec(`find ${manifestsDirForEnv} -type d -name "${dirName}"`, {
        silent: true,
      })
      .stdout.split("\n");
  const manifestsNonCrds = getManifestsWithinDirName("1-manifest");
  const manifestsCrds = getManifestsWithinDirName("0-crd");
  manifestsNonCrds.concat(manifestsCrds).forEach((f) => sh.rm("-rf", f.trim()));

  sh.exec(
    "export PULUMI_CONFIG_PASSPHRASE='' && pulumi stack init --stack dev"
  );

  // Pulumi needs some environment variables set for generating deployments with image tag
  /* `export ${IMAGE_TAG_REACT_WEB}=tag-web export ${IMAGE_TAG_GRAPHQL_MONGO}=tag-mongo`
   */

  sh.exec(
    `
    ${getEnvVarsForScript(environment, imageTags)}
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
      const unsealedSecretManifestFileName = p.basename(
        unsealedSecretManifestPath
      );
      const sealedSecretDir = p.join(appBaseDir, "sealed-secrets");
      const sealedSecretFilePath = p.join(
        sealedSecretDir,
        `sealed-${unsealedSecretManifestFileName}`
      );
      const sealedSecretsControllerName: ResourceName = "sealed-secrets";

      sh.mkdir(sealedSecretDir);

      // TODO: Check the content of the file to confirm if it is actually a secret object
      sh.echo(
        c.blueBright(
          `Generating sealed secret ${unsealedSecretManifestPath} \n to \n ${sealedSecretFilePath}`
        )
      );
      // inject `sealedsecrets.bitnami.com/managed: "true"` into existing secret annotaions to
      // enable sealed secret controller manage existing secrets
      // https://github.com/bitnami-labs/sealed-secrets/blob/main/README.md#managing-existing-secrets
      // sh.exec(`yq -i '.metadata.annotations["sealedsecrets.bitnami.com/managed"] = "true"' ${unsealedSecretManifestPath}`)

      if (sealedSecretFilePath) {
        // Merge into existing sealed secret if it alredy exists. Otherwise, create a fresh one
        // First check if value is empty. i.e empty string or null
        // Delete empty and null secret data
        // sh.exec(`yq eval 'del( .data[] | select( . == "" or . == null) )' dd.yaml`)
        // Encode value of an empty string is Cg==
        const emptyStringInBase64 = "Cg=="
        // -i does the mutation in place
        sh.exec(`yq -i eval 'del( .data[] | select( . == "" or . == null or . == "${emptyStringInBase64}") )' ${unsealedSecretManifestPath}`)
        // sh.exec(`yq e '.metadata.annotations' dd.yaml`)

        //Delete value if it is empty or null
        // Add -i if you want the deletion in place
        // sh.exec(`yq eval 'del(.metadata.annotations["sealedsecrets.bitnami.com/managedq"])' dd.yaml`)

        // Merge only the values that have newly been included in the plain secret to the newly merged sealed secrets
        sh.exec(`kubeseal --controller-name ${sealedSecretsControllerName} < ${unsealedSecretManifestPath} -o yaml --merge-into  ${sealedSecretFilePath}`)
      } else {
        // TODO: Should I delete old sealed secrets before creating new ones?
        const kubeSeal = sh.exec(
          `kubeseal --controller-name ${sealedSecretsControllerName} < ${unsealedSecretManifestPath} -o yaml >${sealedSecretFilePath}`,
          {
            silent: true,
          }
        );
        // const kubeSeal = sh.exec(
        //   `kubeseal --controller-name ${sealedSecretsControllerName} < ${unsealedSecretManifestPath} -o yaml >${sealedSecretFilePath}`,
        //   {
        //     silent: true,
        //   }
        // );

        sh.echo(c.greenBright(kubeSeal.stdout));
        if (kubeSeal.stderr) {
          sh.echo(`Error sealing secrets: ${c.redBright(kubeSeal.stderr)}`);
          sh.exit(1);
          return;
        }
      }


      sh.echo(
        c.greenBright(
          "Successfully generated sealed secret at",
          unsealedSecretManifestPath
        )
      );
    }

    sh.echo(
      c.blueBright(
        `Removing unsealed plain secret manifest ${unsealedSecretManifestPath}`
      )
    );

    // Delete unsealed plain secret if specified
    if (!keepSecretOutputs) {
      sh.rm("-rf", unsealedSecretManifestPath);
    }

    if (!keepSecretInputs) {
      clearUnsealedInputTsSecretFilesContents();
    }
  }
}

export function getEnvVarsForScript(
  environment: Environment,
  imageTags: ImageTags
) {
  const imageEnvVarSetterForPulumi = Object.entries(imageTags)
    .map(([k, v]) => `export ${k}=${v}`)
    .join(" ");
  return `
      ${imageEnvVarSetterForPulumi} 
      export ${ENVIRONMENT}=${environment}  
  `;
}

export function getFilePathsThatMatch({
  contextDir,
  pattern,
}: {
  contextDir: string;
  pattern: string;
}) {
  const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = sh.exec(
    `find ${contextDir} -name "${pattern}"`,
    {
      silent: true,
    }
  );
  const unsealedSecretsFilePathsForEnv =
    UNSEALED_SECRETS_MANIFESTS_FOR_ENV.stdout
      .trim()
      .split("\n")
      .map((s) => s.trim());
  return unsealedSecretsFilePathsForEnv;
}

import { ImageTags } from "./../resources/shared/validations";
import sh from "shelljs";
import { Environment } from "../resources/shared/types/own-types";
import { getImageTagsFromDir } from "./getImageTagsFromDir";
import { manifestsDirForEnv, ENVIRONMENT, ARGV } from "./bootstrap";
import path from "path";
import { getFilePathsThatMatch } from "./generateSealedSecretsManifests";
// function getImageTags() {
//   sh.ls()
// }
/*
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/
export async function generateManifests({
  environment,
  imageTags,
}: {
  environment: Environment;
  imageTags: ImageTags;
}) {
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
}

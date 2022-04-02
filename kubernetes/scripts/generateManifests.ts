import { ImageTags } from "./../resources/shared/validations";
import sh from "shelljs";
import { Environment } from "../resources/shared/types/own-types";
import { getImageTagsFromDir } from "./getImageTagsFromDir";
import { manifestsDirForEnv, ENVIRONMENT, ARGV } from "./script";

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
  // sh.cd(__dirname);
  // sh.exec("npm i");
  sh.exec("npm i");
  sh.rm("-rf", "./login");
  sh.mkdir("./login");
  sh.rm("-rf", manifestsDirForEnv);
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

  const shGenerateManifestsOutput = sh.exec(
    `
      ${imageEnvVarSetterForPulumi} 
      export ${ENVIRONMENT}=${environment}  
      export PULUMI_CONFIG_PASSPHRASE="" 
      pulumi update --yes --skip-preview --stack dev
      `,
    {
      /* silent: true
      /* fatal: true */
    }
  );

  // Used this hack to
  // PULUMI unfortunately seems to push all the logs to stdout. Might patch it if need be
  // const stdout = shGenerateManifestsOutput.stdout;
  // sh.echo(c.greenBright(shGenerateManifestsOutput.stdout));
  // // TODO: There has to be a better way. And open an issue/PR on pulumi repo or patch package locally
  // // This would be sufficient if pulumi would just send error to stdout instead of sending all to stdout
  // if (shGenerateManifestsOutput.stderr) {
  //   sh.echo(c.redBright(shGenerateManifestsOutput.stderr));
  //   sh.exit(1);
  // }
  // const errorText = sh.exec(c.redBright(`${stdout} | grep Error:`));
  // if (errorText) {
  //   sh.echo(
  //     c.redBright(stdout.split(/\r?\n/).find((l) => l.toLocaleLowerCase().includes("error")))
  //   );
  //   // Get the error out. This is a little brittle but well, I need to raise an issue with pulumi
  //   // const err = stdout.substring(stdout.indexOf("Error:"));
  //   // sh.echo(c.redBright(err));
  //   // sh.exit(1);
  // }
}

import { getGeneratedEnvManifestsDir } from "../../resources/shared/manifestsDirectory";
import {
  Environment,
  ResourceName,
} from "../../resources/shared/types/own-types";
import p from "path";
import { getFilePathsThatMatch } from "./shared";
import yargs from "yargs/yargs";
import c from "chalk";
import inquirer from "inquirer";
import sh from "shelljs";
import util from "util";

export const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = "sealed-secrets";

export function getSecretPathsInfo({
  unsealedSecretFilePath,
}: {
  unsealedSecretFilePath: string;
}) {
  const appManifestsDir = p.dirname(unsealedSecretFilePath);
  // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
  // and we want as basedir: kubernetes/manifests/generated/production/applications/graphql-mongo
  const appBaseDir = p.join(appManifestsDir, "..");
  const unsealedSecretFileName = p.basename(unsealedSecretFilePath);
  // TODO: Get this as an argument to the function whch will be prompted on command start
  // if (secretsToUpdate.inclues(unsealedSecretFileName)) {
  // }

  const sealedSecretDir = p.join(appBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
  const sealedSecretFilePath = p.join(
    sealedSecretDir,
    `sealed-${unsealedSecretFileName}`
  );
  return {
    sealedSecretDir,
    sealedSecretFilePath,
    // sealedSecretsControllerName: SEALED_SECRETS_CONTROLLER_NAME,
  } as const;
}

export function getSecretManifestsPaths(environment: Environment): string[] {
  const contextDir = getGeneratedEnvManifestsDir(environment);
  const unsealedSecretsFilePathsForEnv = getFilePathsThatMatch({
    contextDir,
    pattern: "secret-*ml",
  });
  return unsealedSecretsFilePathsForEnv;
}

export const ENVIRONMENTS_ALL: Environment[] = [
  "local",
  "production",
  "staging",
  "development",
];
export async function promptEnvironmentSelection() {
  const choices = ENVIRONMENTS_ALL.flatMap((env) => [
    env,
    new inquirer.Separator(),
  ]);

  const name = "environment";
  const answers: Record<typeof name, Environment> = await inquirer.prompt([
    {
      type: "list",
      name,
      message: c.greenBright("🆘Select the environment ‼️‼️‼️‼️"),
      choices,
      default: ENVIRONMENTS_ALL[0],
      pageSize: 20,
    } as const,
  ]);

  return answers;
}

// export function getSecretEnvironmentArgs() {
//   const ARGV = yargs(process.argv.slice(2))
//     .options({
//       e: {
//         alias: "environment",
//         choices: [
//           "local",
//           "development",
//           "staging",
//           "production",
//         ] as Environment[],
//         describe: "The environment you're generating the manifests for.",
//         demandOption: true,
//       },
//     }).parseSync()
//   return ARGV
// }

export function removeAllPlainSecrets(environment: Environment) {
  getSecretManifestsPaths(environment).map((unsealedSecretFilePath) => {
    sh.echo(
      c.blueBright(
        `Removing unsealed plain secret manifest ${unsealedSecretFilePath}`
      )
    );
    // Delete unsealed plain secret if specified
    sh.rm("-rf", unsealedSecretFilePath);
  });
}

import inquirer from "inquirer";
import chalk from "chalk";

export async function promptSecretsKeepingConfirmations() {
  const keepPlainSecretsInput = "keepPlainSecretsInput";
  const keepUnsealedSecretManifestsOutput = "keepUnsealedSecretManifestsOutput";
  type Key =
    | typeof keepPlainSecretsInput
    | typeof keepUnsealedSecretManifestsOutput;
  const answers = await inquirer.prompt<Record<Key, boolean>>([
    {
      type: "confirm",
      name: keepPlainSecretsInput,
      message: chalk.greenBright(
        `🆘Do you want to keep the plain secrets used for generating the sealed secrets? ‼️‼️‼️‼️`
      ),
      // default: true,
    },
    {
      type: "confirm",
      name: keepUnsealedSecretManifestsOutput,
      message: chalk.greenBright(
        `🆘Do you want to keep the kubernetes secrets manifests generated?
        Note: These should never be pushed to git ‼️‼️‼️‼️`
      ),
      // default: false,
    },
  ]);

  return answers;
}

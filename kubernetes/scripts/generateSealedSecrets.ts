import sh from "shelljs";
import {
    clearUnsealedInputTsSecretFilesContents,
    setupPlainSecretTSFiles,
} from "./secretsManagement/setupSecrets";
import { generateAllSealedSecrets } from "./utils/generateAllSealedSecrets";
import { generateManifests } from "./utils/generateManifests";
import { getImageTagsFromDir } from "./utils/getImageTagsFromDir";
import { promptKubernetesClusterSwitch } from "./utils/promptKubernetesClusterSwitch";
import { promptSecretsKeepingConfirmations } from "./utils/promptSecretsKeepingConfirmations";
import {
    getSecretManifestsPaths,
    getSecretPathsInfo,
    promptEnvironmentSelection,
} from "./utils/sealedSecrets";
import { updateAppSealedSecrets } from "./utils/updateApplicationsSecrets";
import inquirer from "inquirer";
import chalk from "chalk";

async function main() {
    const { generateSecretsOptions } = await promptSealedSecretsMergingOptions();

    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);
    const { keepPlainSecretsInput, keepUnsealedSecretManifestsOutput } =
        await promptSecretsKeepingConfirmations();

    const imageTags = await getImageTagsFromDir();

    await generateManifests({
        environment,
        imageTags,
    });

    // setupPlainSecretTSFiles();


    if (generateSecretsOptions === "Generate all secrets") {
        generateAllSealedSecrets({ environment });
    } else if (generateSecretsOptions === "merge self managed secrets") {
        updateAppSealedSecrets(environment);
    }

    if (!keepPlainSecretsInput) {
        clearUnsealedInputTsSecretFilesContents();
    }

    if (!keepUnsealedSecretManifestsOutput) {
        const removeSecret = (path: string) => sh.rm("-rf", path);
        getSecretManifestsPaths(environment).forEach(removeSecret);
    }
}

main();



export async function promptSealedSecretsMergingOptions() {
    const options = ["merge self managed secrets", "Generate all secrets"] as const;
    const choices = options.flatMap((env) => [env, new inquirer.Separator()]);
    const optionName = "generateSecretsOptions";

    const answers = await inquirer.prompt<Record<typeof optionName, typeof options[number]>>({
        type: "list",
        name: optionName,
        message: chalk.bgRedBright`Sealed secret generation options‼️‼️‼️‼️`,
        choices,
        default: options[0]
    });

    return answers;
}

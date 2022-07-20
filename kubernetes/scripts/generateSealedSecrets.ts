import chalk from 'chalk';
import inquirer from 'inquirer';
import sh from 'shelljs';
import { clearPlainInputTsSecretFilesContents, syncSecretsTsFiles } from './secretsManagement/syncSecretsTsFiles';
import { generateManifests } from './utils/generateManifests';
import { getImageTagsFromDir } from './utils/getImageTagsFromDir';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { generateAllSealedSecrets } from './utils/sealed-secrets/generateAllSealedSecrets';
import { promptSecretsDeletionConfirmations } from './utils/promptSecretsDeletionConfirmations';
// import { promptSecretsKeepingConfirmations } from './utils/promptSecretsKeepingConfirmations';
import { syncAppSealedSecrets } from './utils/syncAppsSecrets';
// import { getSecretManifestsPaths, } from './utils/sealedSecrets';
import { getSecretManifestsPaths, promptEnvironmentSelection } from './utils/shared';
// import { updateAppSealedSecrets } from './utils/updateApplicationsSecrets';

async function main() {
    const { generateSecretsOptions } = await promptSealedSecretsMergingOptions();

    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);
    const { , } = await promptSecretsDeletionConfirmations();

    const imageTags = await getImageTagsFromDir();

    await generateManifests({
        environment,
        imageTags,
    });

    syncSecretsTsFiles();

    if (generateSecretsOptions === 'Generate all secrets') {
        generateAllSealedSecrets({ environment });
    } else if (generateSecretsOptions === 'merge self managed secrets') {
        updateAppSealedSecrets(environment);
    }

    if (!keepPlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (!keepUnsealedSecretManifestsOutput) {
        const removeSecret = (path: string) => sh.rm('-rf', path);
        getSecretManifestsPaths(environment).forEach(removeSecret);
    }
}

main();

export async function promptSealedSecretsMergingOptions() {
    const options = ['merge self managed secrets', 'Generate all secrets'] as const;
    const choices = options.flatMap((env) => [env, new inquirer.Separator()]);
    const optionName = 'generateSecretsOptions';

    const answers = await inquirer.prompt<Record<typeof optionName, typeof options[number]>>({
        type: 'list',
        name: optionName,
        message: chalk.blueBright`Sealed secret generation options‼️‼️‼️‼️`,
        choices,
        default: options[0],
    });

    return answers;
}

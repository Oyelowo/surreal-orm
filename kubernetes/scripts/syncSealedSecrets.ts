import chalk from 'chalk';
import sh from 'shelljs';
import { clearPlainInputTsSecretFilesContents, syncSecretsTsFiles } from './secretsManagement/syncSecretsTsFiles';
import { generateManifests } from './utils/generateManifests';
import { getImageTagsFromDir } from './utils/getImageTagsFromDir';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { promptSecretsDeletionConfirmations } from './utils/promptSecretsDeletionConfirmations';
import { syncAppSealedSecrets } from './utils/syncAppsSecrets';
import { getAllKubeManifestsInfo, getKubeManifestsPaths, promptEnvironmentSelection } from './utils/shared';

async function main() {
    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);
    const { deletePlainSecretsInput, deleteUnsealedSecretManifestsOutput } = await promptSecretsDeletionConfirmations();

    const imageTags = await getImageTagsFromDir();

    await generateManifests({
        environment,
        imageTags,
        allManifestsInfo: getAllKubeManifestsInfo(environment),
    });

    syncSecretsTsFiles();

    const allManifestsInfo = getAllKubeManifestsInfo(environment);
    syncAppSealedSecrets(environment, allManifestsInfo);

    if (deletePlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (deleteUnsealedSecretManifestsOutput) {
        const removeSecret = (path: string) => sh.rm('-rf', path);
        getKubeManifestsPaths({ kind: "Secret", allManifestsInfo }).forEach(removeSecret);
    }
}

main();

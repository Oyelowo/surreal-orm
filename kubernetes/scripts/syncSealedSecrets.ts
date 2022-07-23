import chalk from 'chalk';
import sh from 'shelljs';
import { clearPlainInputTsSecretFilesContents, syncSecretsTsFiles } from './secretsManagement/syncSecretsTsFiles';
import { generateManifests } from './utils/kubeObject/generateManifests';
import { getImageTagsFromDir } from './utils/getImageTagsFromDir';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { promptSecretsDeletionConfirmations } from './utils/promptSecretsDeletionConfirmations';
import { KubeObject } from './utils/kubeObject/kubeObject';
import { promptEnvironmentSelection } from './utils/shared';

async function main() {
    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);
    const { deletePlainSecretsInput, deleteUnsealedSecretManifestsOutput } = await promptSecretsDeletionConfirmations();

    const imageTags = await getImageTagsFromDir();

    const kubeObject = new KubeObject(environment);

    await kubeObject.generateManifests();

    syncSecretsTsFiles();

    // This requires the cluster to be on
    kubeObject.syncSealedSecretsWithPrompt();

    if (deletePlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (deleteUnsealedSecretManifestsOutput) {
        kubeObject.getOfAKind('Secret').forEach(({ path }) => {
            sh.rm('-rf', path);
        });
    }
}

main();

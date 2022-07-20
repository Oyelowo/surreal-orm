import sh from 'shelljs';
import { clearPlainInputTsSecretFilesContents } from './secretsManagement/syncSecretsTsFiles';
import { bootstrapCluster } from './utils/bootstrapCluster';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { promptSecretsDeletionConfirmations } from './utils/promptSecretsDeletionConfirmations';
import { promptEnvironmentSelection } from './utils/shared';
import { getSecretManifestsPaths, } from './utils/shared';

async function main() {
    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);

    const { deletePlainSecretsInput, deleteUnsealedSecretManifestsOutput } = await promptSecretsDeletionConfirmations();

    await bootstrapCluster(environment);

    if (deletePlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (deleteUnsealedSecretManifestsOutput) {
        const removeSecret = (path: string) => sh.rm('-rf', path);
        getSecretManifestsPaths(environment).forEach(removeSecret);
    }
}

main();

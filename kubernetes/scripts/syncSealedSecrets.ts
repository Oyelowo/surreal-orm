import sh from 'shelljs';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { promptSecretsDeletionConfirmations } from './utils/promptSecretsDeletionConfirmations';
import { KubeObject } from './utils/kubeObject/kubeObject';
import { promptEnvironmentSelection } from './utils/shared';
import { PlainSecretJsonConfig } from './utils/plainSecretJsonConfig';

async function main() {
    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);
    const { deletePlainSecretsInput, deleteUnsealedSecretManifestsOutput } = await promptSecretsDeletionConfirmations();

    const kubeObject = new KubeObject(environment);

    await kubeObject.generateManifests();
    PlainSecretJsonConfig.syncAll();

    // This requires the cluster to be on and switch to its context
    kubeObject.syncSealedSecretsWithPrompt();

    if (deletePlainSecretsInput) {
        PlainSecretJsonConfig.emptyAll();
    }

    if (deleteUnsealedSecretManifestsOutput) {
        KubeObject.getOfAKind('Secret').forEach(({ path }) => {
            sh.rm('-rf', path);
        });
    }
}

main();

import sh from 'shelljs';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch.js';
import { promptSecretsDeletionConfirmations } from './utils/promptSecretsDeletionConfirmations.js';
import { KubeObject } from './utils/kubeObject/kubeObject.js';
import { promptEnvironmentSelection } from './utils/shared.js';
import { PlainSecretJsonConfig } from './utils/plainSecretJsonConfig.js';

async function main() {
    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);
    const { deletPlainJsonSecretsInput, deleteUnsealedSecretManifestsOutput } =
        await promptSecretsDeletionConfirmations();

    const kubeObject = new KubeObject(environment);

    await kubeObject.generateManifests();
    PlainSecretJsonConfig.syncAll();

    // This requires the cluster to be on and switch to its context
    await kubeObject.syncSealedSecretsWithPrompt();

    if (deletPlainJsonSecretsInput) {
        PlainSecretJsonConfig.emptyValues(environment);
    }

    if (deleteUnsealedSecretManifestsOutput) {
        kubeObject.getOfAKind('Secret').forEach(({ path }) => {
            sh.rm('-rf', path);
        });
    }
}

main();

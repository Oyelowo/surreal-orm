import { clearPlainInputTsSecretFilesContents } from "./secretsManagement/setupSecrets";
import { bootstrapCluster } from "./utils/bootstrapCluster";
import { promptKubernetesClusterSwitch } from "./utils/promptKubernetesClusterSwitch";
import {
    promptEnvironmentSelection,
    getSecretManifestsPaths,
} from "./utils/sealedSecrets";
import sh from "shelljs";
import { promptSecretsKeepingConfirmations } from "./utils/promptSecretsKeepingConfirmations";


async function main() {
    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);
    const { keepPlainSecretsInput, keepUnsealedSecretManifestsOutput } = await promptSecretsKeepingConfirmations();

    await bootstrapCluster(environment);

    if (!keepPlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (!keepUnsealedSecretManifestsOutput) {
        const removeSecret = (path: string) => sh.rm("-rf", path);
        getSecretManifestsPaths(environment).forEach(removeSecret);
    }
};

main()

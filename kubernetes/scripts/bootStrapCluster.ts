import { clearUnsealedInputTsSecretFilesContents } from "./secretsManagement/setupSecrets";
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
  const { keepPlainSecretsInput, keepUnsealedSecretManifestsOutput } =
    await promptSecretsKeepingConfirmations();
  console.log({ keepPlainSecretsInput, keepUnsealedSecretManifestsOutput });

  await bootstrapCluster(environment);

  if (!keepPlainSecretsInput) {
    clearUnsealedInputTsSecretFilesContents();
  }

  if (!keepUnsealedSecretManifestsOutput) {
    getSecretManifestsPaths(environment).forEach((SecretPath) => {
      sh.rm("-rf", SecretPath);
    });
  }
};

main()


import { generateAllSealedSecrets } from "./utils/generateAllSealedSecrets";
import { promptEnvironmentSelection } from "./utils/sealedSecrets";


// if (!keepSecretInputs) {
//   clearUnsealedInputTsSecretFilesContents();
// }



// // const args = getSecretEnvironmentArgs()
(async function () {
    const args = await promptEnvironmentSelection();
    generateAllSealedSecrets({ environment: args.environment });
})();

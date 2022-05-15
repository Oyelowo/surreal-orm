import { promptKubernetesClusterSwitch } from "./promptKubernetesClusterSwitch";
import {
  getAllUnsealedSecretsPaths,
  getSecretPathsInfo,
  SEALED_SECRETS_CONTROLLER_NAME,
} from "./sealedSecrets";
import { SecretTemplate } from "../../resources/shared/types/SecretTemplate";
import { SealedSecretTemplate } from "../../resources/shared/types/sealedSecretTemplate";
import {
  getGeneratedEnvManifestsDir,
  ResourceName,
} from "../../resources/shared/manifestsDirectory";
import { ImageTags } from "../../resources/shared/validations";
import sh, { ShellString } from "shelljs";
import { Environment } from "../../resources/shared/types/own-types";
import p from "path";
import c from "chalk";
import { clearUnsealedInputTsSecretFilesContents } from "../secretsManagement/setupSecrets";
import yaml from "js-yaml";
import fs from "fs";
import { getFilePathsThatMatch } from "./shared";

async function updateSelfManagedSecrets() {
  await promptKubernetesClusterSwitch("production");
  const unsealedSecretsFilePathsForEnv = getAllUnsealedSecretsPaths("production");
  console.log("unsealedSecretsFilePathsForEnv", unsealedSecretsFilePathsForEnv);

  for (const unsealedSecretFilePath of unsealedSecretsFilePathsForEnv) {
    const { sealedSecretDir, sealedSecretFilePath } = getSecretPathsInfo({
      unsealedSecretFilePath,
    });

    mergeSecretToSealedSecret({
      unsealedSecretFilePath,
      sealedSecretFilePath,
    });
  }
}

type MergeProps = {
  unsealedSecretFilePath: string;
  sealedSecretFilePath: string;
  // sealedSecretsControllerName: string;
};

function mergeSecretToSealedSecret({
  unsealedSecretFilePath,
  sealedSecretFilePath,
}: MergeProps): void {
  const emptyStringInBase64 = "Cg==";
  const unsealedSecretJsonData: SecretTemplate = yaml.load(
    fs.readFileSync(unsealedSecretFilePath, { encoding: "utf-8" })
  ) as SecretTemplate;

  const removeEmptyValue = ([_, value]: [string, string]) =>
    !(value === "" || value === null || value === emptyStringInBase64);
  const sealValue = ([key, value]: [string, string]) => [
    key,
    sh.exec(
      `echo -n ${value} | kubeseal --controller-name=${SEALED_SECRETS_CONTROLLER_NAME} \
         --raw --from-file=/dev/stdin --namespace ${unsealedSecretJsonData.metadata.namespace} \
          --name ${unsealedSecretJsonData.metadata.name}`
    ),
  ];

  const { stringData, data } = unsealedSecretJsonData;
  const dataToSeal = stringData ?? data ?? {};
  const filteredSealedSecretsData = Object.fromEntries(
    Object.entries(dataToSeal).filter(removeEmptyValue).map(sealValue)
  );

  const existingSealedSecretJsonData: SealedSecretTemplate = yaml.load(
    fs.readFileSync(sealedSecretFilePath, { encoding: "utf-8" })
  ) as SealedSecretTemplate;

  const updatedSealedSecrets: SealedSecretTemplate = {
    ...existingSealedSecretJsonData,
    spec: {
      encryptedData: {
        ...existingSealedSecretJsonData?.spec?.encryptedData,
        ...filteredSealedSecretsData,
      },
      template: {
        ...existingSealedSecretJsonData?.spec?.template,
        data: null,
        metadata: unsealedSecretJsonData.metadata,
        type: unsealedSecretJsonData.type,
      },
    },
  };

  sh.exec(
    `echo '${yaml.dump(updatedSealedSecrets)}' > ${sealedSecretFilePath}`
  );

  // Something as simple as this would have worked but kubeseal doesnt handle merging properly
  // When there is a key in the new secret but not in the existing sealed secret, it throws an error
  // sh.exec(`echo '${JSON.stringify(Data)}' | kubeseal --controller-name ${sealedSecretsControllerName} -o yaml --merge-into  ${sealedSecretFilePath}`)
}


updateSelfManagedSecrets();

/* 
// TODO: Prompt user to specify which apps secrets they're looking to update
export async function regenerateSealedSecretsManifests({
  environment,
  keepSecretInputs,
  keepSecretOutputs,
  regenerateSealedSecrets,
}: GenSealedSecretsProps) {
  // const contextDir = p.join(__dirname, "..", "manifests", "generated", environment);
  const contextDir = getGeneratedEnvManifestsDir(environment);
  const unsealedSecretsFilePathsForEnv = getFilePathsThatMatch({
    contextDir,
    pattern: "secret-*ml",
  });

  for (const unsealedSecretFilePath of unsealedSecretsFilePathsForEnv) {
    const appManifestsDir = p.dirname(unsealedSecretFilePath);
    // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
    // and we want as basedir: kubernetes/manifests/generated/production/applications/graphql-mongo
    const appBaseDir = p.join(appManifestsDir, "..");
    const unsealedSecretFileName = p.basename(unsealedSecretFilePath);
    // TODO: Get this as an argument to the function whch will be prompted on command start
    // if (secretsToUpdate.inclues(unsealedSecretFileName)) {
    // }
    const sealedSecretsControllerName: ResourceName = "sealed-secrets";
    const sealedSecretDir = p.join(appBaseDir, sealedSecretsControllerName);
    const sealedSecretFilePath = p.join(
      sealedSecretDir,
      `sealed-${unsealedSecretFileName}`
    );

    sh.mkdir(sealedSecretDir);

    // TODO: Check the content of the file to confirm if it is actually a secret object
    sh.echo(
      c.blueBright(
        `Generating sealed secret ${unsealedSecretFilePath} \n to \n ${sealedSecretFilePath}`
      )
    );

    const isEmpty = await isFileEmpty(sealedSecretFilePath);
    console.log("isEmpty", isEmpty);
    if (sealedSecretFilePath) {
      mergeSecretToSealedSecret({
        unsealedSecretFilePath,
        sealedSecretsControllerName,
        sealedSecretFilePath,
      });
    } else {
      // TODO: Should I delete old sealed secrets before creating new ones?
      const kubeSeal = sh.exec(
        `kubeseal --controller-name ${sealedSecretsControllerName} < ${unsealedSecretFilePath} -o yaml >${sealedSecretFilePath}`,
        {
          silent: true,
        }
      );

      sh.echo(c.greenBright(kubeSeal.stdout));
      if (kubeSeal.stderr) {
        sh.echo(`Error sealing secrets: ${c.redBright(kubeSeal.stderr)}`);
        sh.exit(1);
        return;
      }
    }

    sh.echo(
      c.greenBright(
        "Successfully generated sealed secret at",
        unsealedSecretFilePath
      )
    );
  }
}


*/

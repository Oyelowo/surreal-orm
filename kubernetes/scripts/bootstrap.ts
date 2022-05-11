#!/usr/bin / env ts - node

import { helmChartsInfo } from "./../resources/shared/helmChartInfo";
import { namespaceNames } from "../resources/namespaces/util";
import {
  getArgocdParentApplicationsPath,
  getResourceAbsolutePath,
  ResourceName,
} from "./../resources/shared/manifestsDirectory";

/* 
TODO: ADD INSTRUCTION ON HOW THIS WORKS
*/
// TODO: Maybe setup base argocd app that deploys other argocd apps?
import { ArgumentTypes } from "../../typescript/apps/web/utils/typescript";
// TODO: Allow the selections of applications to regenerate secret for. This should be done with inquirer prompt.
// This would read the name of the app = name of deployment in manifests to determine the sealed secrets  to regenerate and override
import path from "path";
import glob from "glob";
import prompt from "prompt";
import sh from "shelljs";
import util from "util";
import yargs from "yargs/yargs";

import { Environment } from "../resources/shared/types/own-types";
import { getEnvironmentVariables } from "../resources/shared/validations";
import { setupUnsealedSecretFiles } from "./secretsManagement/setupSecrets";
import {
  generateManifests,
  regenerateSealedSecretsManifests,
} from "./generateManifests";
import { getImageTagsFromDir } from "./getImageTagsFromDir";
import { promptKubernetesClusterSwitch } from "./promptKubernetesClusterSwitch";
import { getGeneratedEnvManifestsDir } from "../resources/shared/manifestsDirectory";

// TODO: Use prompt to ask for which cluster this should be used with for the sealed secrets controller
// npm i inquirer
type EnvName = keyof ReturnType<typeof getEnvironmentVariables>;
export const globAsync = util.promisify(glob);
const promptGetAsync = util.promisify(prompt.get);
export const ENVIRONMENT: EnvName = "ENVIRONMENT";
// find ./kubernetes -name "secret*ml"
const yesOrNoOptions = ["y", "yes", "no", "n"] as const;
type YesOrNoOptions = typeof yesOrNoOptions[number];

export const ARGV = yargs(process.argv.slice(2))
  .options({
    e: {
      alias: "environment",
      choices: [
        "local",
        "development",
        "staging",
        "production",
      ] as Environment[],
      describe: "The environment you're generating the manifests for.",
      demandOption: true,
    },
    gss: {
      alias: "generate-sealed-secrets",
      choices: yesOrNoOptions,
      // default: "no" as YesOrNoOptions,
      describe:
        "Generate sealed secrets manifests from generated plain secrets manifests",
      demandOption: true,
    },
    kuso: {
      alias: "keep-unsealed-secrets-output",
      choices: yesOrNoOptions,
      default: "no" as YesOrNoOptions,
      describe:
        "Keep unsealed secrets output generated plain kubernetes manifests",
      // demandOption: true,
    },
    kusi: {
      alias: "keep-unsealed-secrets-input",
      choices: yesOrNoOptions,
      // default: "no" as YesOrNoOptions,
      describe:
        "Keep unsealed secrets inputs plain configs used to generate kubernetes secrets manifests",
      demandOption: true,
    },
  } as const)
  .parseSync();

const manifestsDirForEnv = getGeneratedEnvManifestsDir(ARGV.e);
// export const manifestsDirForEnv = path.join("manifests", "generated", ARGV.e);

async function bootstrap() {
  // TODO:
  // if bootsrapping i.e fresh cluster, delete the entire folder for that environment
  const yes: YesOrNoOptions[] = ["yes", "y"];

  const generateSealedSecrets = yes.includes(ARGV.gss);
  const imageTags = await getImageTagsFromDir();

  if (!generateSealedSecrets) {
    await generateManifests({
      environment: ARGV.e,
      imageTags,
    });
    return;
  }

  await promptKubernetesClusterSwitch();

  await generateManifests({
    environment: ARGV.e,
    imageTags,
  });

  setupUnsealedSecretFiles();

  /*
       This requires the above step with initial cluster setup making sure that the sealed secret controller is
       running in the cluster */

  // # Apply namespace first
  applyResourceManifests("namespace-names")

  // # Apply setups with sealed secret controller
  applyResourceManifests("sealed-secrets")

  const sealedSecretsName: ResourceName = "sealed-secrets";
  // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
  sh.exec(
    `kubectl rollout status deployment/${sealedSecretsName} -n=${namespaceNames.kubeSystem}`
  );

  // # Apply setups with cert-manager controller
  applyResourceManifests("cert-manager")

  // # Wait for cert-manager and cert-manager-trust controllers to be in running phase so that we can use it to encrypt secrets
  const { certManager, certManagerTrust } = helmChartsInfo.jetspackRepo;
  sh.exec(
    `kubectl rollout status deployment/${certManager.chart} -n=${namespaceNames.certManager}`
  );
  sh.exec(
    `kubectl rollout status deployment/${certManagerTrust.chart} -n=${namespaceNames.certManager}`
  );

  // # Apply setups with linkerd controller
  applyResourceManifests("linkerd");
  applyResourceManifests("linkerd-viz");

  // TODO: separate sealed secrets deletion step
  await regenerateSealedSecretsManifests({
    environment: ARGV.e,
    regenerateSealedSecrets: generateSealedSecrets,
    keepSecretInputs: yes.includes(ARGV.kusi),
    keepSecretOutputs: yes.includes(ARGV.kuso),
  });

  // TODO: could conditionally check the installation of argocd also cos it may not be necessary for local dev
  applyResourceManifests("argocd")
  // TODO: Split bootstrap process from restart from update
  sh.exec(`kubectl -n argocd rollout restart deployment argocd-argo-cd-server`);


  // TODO: Only apply this in non prod environment
  sh.exec(`kubectl apply -R -f ${getArgocdParentApplicationsPath(ARGV.e)}`);
}

bootstrap();



function applyResourceManifests(resourceName: ResourceName) {
  const resourceDir = getResourceAbsolutePath(resourceName, ARGV.e);
  const applyManifests = (dir: string) => sh.exec(`kubectl apply -R -f  ${path.join(resourceDir, dir)}`);
  ["sealed-secrets", "0-crd", "1-manifest"].forEach(applyManifests)


  // sh.exec(`kubectl apply -R -f  ${resourceDir}/sealed-secrets`);
  // sh.exec(`kubectl apply -R -f  ${resourceDir}/0-crd`);
  // sh.exec(`kubectl apply -R -f  ${resourceDir}/1-manifest`);
}


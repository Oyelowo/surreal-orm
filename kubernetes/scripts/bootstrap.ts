#!/usr/bin / env ts - node

import { helmChartsInfo } from './../resources/shared/helmChartInfo';
import { namespaceNames } from '../resources/namespaces/util';
import { getResourceAbsolutePath, getResourceProperties, ResourceName } from "./../resources/shared/manifestsDirectory";

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
import { generateManifests, regenerateSealedSecretsManifests } from "./generateManifests";
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
      choices: ["local", "development", "staging", "production"] as Environment[],
      describe: "The environment you're generating the manifests for.",
      demandOption: true,
    },
    gss: {
      alias: "generate-sealed-secrets",
      choices: yesOrNoOptions,
      // default: "no" as YesOrNoOptions,
      describe: "Generate sealed secrets manifests from generated plain secrets manifests",
      demandOption: true,
    },
    kuso: {
      alias: "keep-unsealed-secrets-output",
      choices: yesOrNoOptions,
      default: "no" as YesOrNoOptions,
      describe: "Keep unsealed secrets output generated plain kubernetes manifests",
      // demandOption: true,
    },
    kusi: {
      alias: "keep-unsealed-secrets-input",
      choices: yesOrNoOptions,
      // default: "no" as YesOrNoOptions,
      describe: "Keep unsealed secrets inputs plain configs used to generate kubernetes secrets manifests",
      demandOption: true,
    },
  } as const)
  .parseSync();

const manifestsDirForEnv = getGeneratedEnvManifestsDir(ARGV.e);
// export const manifestsDirForEnv = path.join("manifests", "generated", ARGV.e);

async function bootstrap() {
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
  // TODO: Use a function to get and share this with manifestDirectory.ts module
  sh.exec(`kubectl apply -R -f ${getResourceAbsolutePath("namespace-names", ARGV.e)}`);
  // sh.exec(`kubectl apply -R -f ${manifestsDirForEnv}/namespaces`);

  // # Apply setups with sealed secret controller

  sh.exec(`kubectl apply -R -f  ${getResourceAbsolutePath("sealed-secrets", ARGV.e)}`);

  const sealedSecretsName: ResourceName = "sealed-secrets";
  // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
  sh.exec(`kubectl rollout status deployment/${sealedSecretsName} -n=${namespaceNames.kubeSystem}`);


  // # Apply setups with cert-manager controller
  const certManagerDir = getResourceAbsolutePath("cert-manager", ARGV.e)
  sh.exec(`kubectl apply -R -f  ${certManagerDir}/0-crd`);
  sh.exec(`kubectl apply -R -f  ${certManagerDir}/1-manifest`);

  // # Wait for cert-manager and cert-manager-trust controllers to be in running phase so that we can use it to encrypt secrets
  const { certManager, certManagerTrust } = helmChartsInfo.jetspackRepo
  sh.exec(`kubectl rollout status deployment/${certManager.chart} -n=${namespaceNames.certManager}`);
  sh.exec(`kubectl rollout status deployment/${certManagerTrust.chart} -n=${namespaceNames.certManager}`);


  // # Apply setups with linkerd controller
  const linkerdDir = getResourceAbsolutePath("linkerd", ARGV.e)
  sh.exec(`kubectl apply -R -f  ${linkerdDir}/sealed-secrets`);
  sh.exec(`kubectl apply -R -f  ${linkerdDir}/0-crd`);
  sh.exec(`kubectl apply -R -f  ${linkerdDir}/1-manifest`);

  const linkerdVizDir = getResourceAbsolutePath("linkerd-viz", ARGV.e)
  sh.exec(`kubectl apply -R -f  ${linkerdVizDir}/sealed-secrets`);
  sh.exec(`kubectl apply -R -f  ${linkerdVizDir}/0-crd`);
  sh.exec(`kubectl apply -R -f  ${linkerdVizDir}/1-manifest`);

return
  // TODO: separate sealed secrets deletion step
  await regenerateSealedSecretsManifests({
    environment: ARGV.e,
    regenerateSealedSecrets: generateSealedSecrets,
    keepSecretInputs: yes.includes(ARGV.kusi),
    keepSecretOutputs: yes.includes(ARGV.kuso),
  });

  // TODO: could conditionally check the installation of argocd also cos it may not be necessary for local dev
  // sh.exec(`kubectl apply -f ${getArgocdControllerDir(ARGV.e)}/sealed-secrets`);
  // sh.exec(`kubectl apply -R -f ${getArgocdControllerDir(ARGV.e)}`);
  const argocdDirController = getResourceAbsolutePath("argocd", ARGV.e)
  sh.exec(`kubectl apply -f ${argocdDirController}/0-crd`);
  sh.exec(`kubectl apply -f ${argocdDirController}/1-manifest`);
  // TODO: Split bootstrap process from restart from update
  sh.exec(`kubectl -n argocd rollout restart deployment argocd-argo-cd-server`);

  // sh.exec(`kubectl apply -R -f ${getIngressControllerDir(ARGV.e)}`);
  // sh.exec(`kubectl apply -R -f ${getCertManagerControllerDir(ARGV.e)}`);
  // // sh.exec(`kubectl apply -R -f ${getLinkerd2Dir(ARGV.e)}`);
  // sh.exec(`kubectl apply -R -f ${getLinkerd2Dir(ARGV.e)}/0-crd`);
  // sh.exec(`kubectl apply -R -f ${getLinkerd2Dir(ARGV.e)}/1-manifest`);

  // sh.exec(`kubectl apply -R -f ${getLinkerdVizDir(ARGV.e)}`);

  // TODO: PUT THE BASE HERE
  // sh.exec(`kubectl apply -R -f ${getResourceProperties("services", ARGV.e)}`);
  // sh.exec(`kubectl apply -R -f ${getArgocdServicesApplicationsDir(ARGV.e)}`);
}

bootstrap();

#!/usr/bin/env node

/* 
TODO: ADD INSTRUCTION ON HOW THIS WORKS
*/
import { ArgumentTypes } from "./../../typescript/apps/web/utils/typescript";

import path from "path";
import glob from "glob";
import prompt from "prompt";
import sh from "shelljs";
import util from "util";
import yargs from "yargs/yargs";
import c from "chalk";

import { Environment } from "./../resources/shared/types/own-types";
import { getEnvironmentVariables } from "../resources/shared/validations";
import {
  clearUnsealedInputTsSecretFilesContents,
  setupUnsealedSecretFiles,
} from "../secretsManagement/setupSecrets";
import {getImageTagsFromDir} from "./kk"
import { getManifestsOutputDirectory } from "../resources/shared";
// TODO: Use prompt to ask for which cluster this should be used with for the sealed secrets controller
// npm i inquirer
type EnvName = keyof ReturnType<typeof getEnvironmentVariables>;
const globAsync = util.promisify(glob);
const promptGetAsync = util.promisify(prompt.get);
const IMAGE_TAG_REACT_WEB: EnvName = "IMAGE_TAG_REACT_WEB";
const IMAGE_TAG_GRAPHQL_MONGO: EnvName = "IMAGE_TAG_GRAPHQL_MONGO";
const IMAGE_TAG_GRPC_MONGO: EnvName = "IMAGE_TAG_GRPC_MONGO";
const IMAGE_TAG_GRAPHQL_POSTGRES: EnvName = "IMAGE_TAG_GRAPHQL_POSTGRES";
const ENVIRONMENT: EnvName = "ENVIRONMENT";
const TEMPORARY_DIR: EnvName = "TEMPORARY_DIR";
const MANIFESTS_DIR = path.join(__dirname, "manifests");
const SEALED_SECRETS_BASE_DIR = path.join(MANIFESTS_DIR, "sealed-secrets");

const yesOrNoOptions = ["y", "yes", "no", "n"] as const;
type YesOrNoOptions = typeof yesOrNoOptions[number];

const ARGV = yargs(process.argv.slice(2))
  .options({
    t: {
      type: "string",
      alias: "tag",
      demandOption: true,
      describe:
        "The docker image tag. Right now, all apps share same image tag which should be local for local environment and github sha for other environments",
    },
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
      describe:
        "Keep unsealed secrets inputs plain configs used to generate kubernetes secrets manifests",
      demandOption: true,
    },
  } as const)
  .parseSync();

// const manifestsDirForEnv = getManifestsOutputDirectory(ARGV.e);
const manifestsDirForEnv = path.join("manifests", "generated", ARGV.e);

prompt.override = ARGV;
prompt.start();

// function getImageTags() {
//   sh.ls()
// }

/* 
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/
async function generateManifests() {
  // sh.cd(__dirname);
  // sh.exec("npm i");

  sh.exec("npm i");
  sh.rm("-rf", "./login");
  sh.mkdir("./login");
  sh.rm("-rf", manifestsDirForEnv);
  sh.exec("pulumi login file://login");
  sh.exec("export PULUMI_CONFIG_PASSPHRASE='' && pulumi stack init --stack dev");

  // image tag. All apps share same tag for now
  const imageTags = await getImageTagsFromDir()
  // Pulumi needs some environment variables set for generating deployments with image tag 
  /* `export ${IMAGE_TAG_REACT_WEB}="${ARGV.t}" && \ `
     `export ${IMAGE_TAG_REACT_WEB}="${ARGV.t}" && \ ` 
     */
  const imageSetterForPulumi = Object.entries(imageTags).map(([k,v]) => `export ${k}=${v} && '\'`) 
  const shGenerateManifestsOutput = sh.exec(
    `
      export ${IMAGE_TAG_REACT_WEB}="${ARGV.t}" && \
      export ${IMAGE_TAG_GRAPHQL_MONGO}="${ARGV.t}" && \
      export ${IMAGE_TAG_GRPC_MONGO}="${ARGV.t}" && \
      export ${IMAGE_TAG_GRAPHQL_POSTGRES}="${ARGV.t}" && \
      export ${ENVIRONMENT}="${ARGV.e}" && \
      export PULUMI_CONFIG_PASSPHRASE=""
      pulumi update --yes --skip-preview --stack dev
      `,
    {
      /* silent: true
      /* fatal: true */
    }
  );

  // Used this hack to
  // PULUMI unfortunately seems to push all the logs to stdout. Might patch it if need be
  // const stdout = shGenerateManifestsOutput.stdout;
  // sh.echo(c.greenBright(shGenerateManifestsOutput.stdout));

  // // TODO: There has to be a better way. And open an issue/PR on pulumi repo or patch package locally
  // // This would be sufficient if pulumi would just send error to stdout instead of sending all to stdout
  // if (shGenerateManifestsOutput.stderr) {
  //   sh.echo(c.redBright(shGenerateManifestsOutput.stderr));
  //   sh.exit(1);
  // }

  // const errorText = sh.exec(c.redBright(`${stdout} | grep Error:`));
  // if (errorText) {
  //   sh.echo(
  //     c.redBright(stdout.split(/\r?\n/).find((l) => l.toLocaleLowerCase().includes("error")))
  //   );
  //   // Get the error out. This is a little brittle but well, I need to raise an issue with pulumi
  //   // const err = stdout.substring(stdout.indexOf("Error:"));
  //   // sh.echo(c.redBright(err));
  //   // sh.exit(1);
  // }
}

/* 
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
const SEALED_SECRETS_DIR_FOR_ENV = `${SEALED_SECRETS_BASE_DIR}/${ARGV.e}`;
type GenSealedSecretsProps = {
  environment: Environment;
  keepSecretOutputs: boolean;
  keepSecretInputs: boolean;
  generateSealedSecrets: boolean;
};

async function generateSealedSecretsManifests({
  environment,
  keepSecretInputs: keepSecretInPuts,
  keepSecretOutputs,
  generateSealedSecrets,
}: GenSealedSecretsProps) {
  const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = path.join(
    MANIFESTS_DIR,
    "generated",
    environment,
    "/**/**/**secret-*ml"
  );

  const unsealedSecretsFilePathsForEnv = await globAsync(UNSEALED_SECRETS_MANIFESTS_FOR_ENV, {
    dot: true,
  });

  unsealedSecretsFilePathsForEnv.forEach((unsealedSecretPath) => {
    if (generateSealedSecrets) {
      sh.echo(c.blueBright("Generating sealed secret from", unsealedSecretPath));

      const secretName = path.basename(unsealedSecretPath);
      const sealedSecretFilePath = `${SEALED_SECRETS_DIR_FOR_ENV}/${secretName}`;

      sh.echo(c.blueBright(`Create ${SEALED_SECRETS_DIR_FOR_ENV} if it does not exists`));
      sh.mkdir("-p", SEALED_SECRETS_DIR_FOR_ENV);

      const kubeSeal = sh.exec(`kubeseal <${unsealedSecretPath} -o yaml >${sealedSecretFilePath}`, {
        silent: true,
      });

      sh.echo(c.greenBright(kubeSeal.stdout));
      if (kubeSeal.stderr) {
        sh.echo(`Error sealing secrets: ${c.redBright(kubeSeal.stderr)}`);
        sh.exit(1);
        return;
      }

      sh.echo(c.greenBright("Successfully generated sealed secret at", unsealedSecretPath));
    }

    sh.echo(c.blueBright(`Removing unsealed plain secret manifest ${unsealedSecretPath}`));

    if (!keepSecretOutputs) {
      sh.rm("-rf", unsealedSecretPath);
    }

    if (!keepSecretInPuts) {
      clearUnsealedInputTsSecretFilesContents();
    }
  });
}

function doInitialClusterSetup() {
  // # Apply namespace first
  // TODO: Use a function to get and share this with manifestDirectory.ts module
  sh.exec(`kubectl apply -R -f ${manifestsDirForEnv}/namespaces`);

  // # Apply setups with sealed secret controller
  sh.exec(`kubectl apply -R -f  ${manifestsDirForEnv}/cluster-setup`);

  // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
  sh.exec(`kubectl rollout status deployment/sealed-secrets-controller -n=kube-system`);
}

async function bootstrap() {
  setupUnsealedSecretFiles();

  await generateManifests();

  const yes: YesOrNoOptions[] = ["yes", "y"];

  await generateSealedSecretsManifests({
    keepSecretOutputs: yes.includes(ARGV.kuso),
    keepSecretInputs: yes.includes(ARGV.kusi),
    generateSealedSecrets: yes.includes(ARGV.gss),
    environment: ARGV.e,
  });

  if (yes.includes(ARGV.gss)) {
    /* 
       This requires the above step with initial cluster setup making sure that the sealed secret controller is
       running in the cluster */
    doInitialClusterSetup();

    // TODO: could conditionally check the installation of argocd also cos it may not be necessary for local dev
    sh.exec(`kubectl apply -f ${manifestsDirForEnv}/argocd/0-crd`);
    sh.exec(`kubectl apply -f ${manifestsDirForEnv}/argocd/1-manifest`);
    sh.exec(`kubectl apply -f ${SEALED_SECRETS_DIR_FOR_ENV}`);
  }
}

bootstrap();

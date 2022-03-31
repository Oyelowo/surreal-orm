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
import { environmentVariables } from "../resources/shared/validations";
import {
  clearUnsealedInputTsSecretFilesContents,
  setupUnsealedSecretFiles,
} from "../secretsManagement/setupSecrets";
import { getManifestsOutputDirectory } from "../resources/shared";

type EnvName = keyof typeof environmentVariables;
const globAsync = util.promisify(glob);
const promptGetAsync = util.promisify(prompt.get);
const IMAGE_TAG_REACT_WEB: EnvName = "IMAGE_TAG_REACT_WEB";
const IMAGE_TAG_GRAPHQL_MONGO: EnvName = "IMAGE_TAG_GRAPHQL_MONGO";
const IMAGE_TAG_GRPC_MONGO: EnvName = "IMAGE_TAG_GRPC_MONGO";
const IMAGE_TAG_GRAPHQL_POSTGRES: EnvName = "IMAGE_TAG_GRAPHQL_POSTGRES";
const ENVIRONMENT: EnvName = "ENVIRONMENT";
const TEMPORARY_DIR: EnvName = "TEMPORARY_DIR";
const MANIFESTS_DIR = "manifests";
const SEALED_SECRETS_BASE_DIR = path.join(MANIFESTS_DIR, "sealed-secrets");

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
      type: "boolean",
      default: false,
      describe: "Generate sealed secrets manifests from generated plain secrets manifests",
      demandOption: true,
    },
    kuso: {
      alias: "keep-unsealed-secrets",
      type: "boolean",
      // default: true,
      describe: "Keep unsealed secrets output generated plain kubernetes manifests",
      // demandOption: true,
    },
    kusi: {
      alias: "keep-unsealed-secrets",
      type: "boolean",
      // default: true,
      describe:
        "Keep unsealed secrets inputs plain configs used to generate kubernetes secrets manifests",
      // demandOption: true,
    },
  } as const)
  .parseSync();

// const manifestsDirForEnv = getManifestsOutputDirectory(ARGV.e);
const manifestsDirForEnv = path.join("manifests", "generated", ARGV.e);

prompt.override = ARGV;
prompt.start();

/* 
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/
async function generateManifests() {
  // sh.cd(__dirname);
  // sh.exec("npm i");
  setupUnsealedSecretFiles();

  sh.exec("npm i");
  sh.rm("-rf", "./login");
  sh.mkdir("./login");
  sh.rm("-rf", manifestsDirForEnv);
  sh.exec("pulumi login file://login");
  sh.exec("export PULUMI_CONFIG_PASSPHRASE='' && pulumi stack init --stack dev");

  // image tag. All apps share same tag for now
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
    { silent: true }
  );
  // sh.echo(c.greenBright(shGenerateManifestsOutput.stdout));
  if (shGenerateManifestsOutput.stderr) {
    sh.echo(c.redBright(shGenerateManifestsOutput.stderr));
    sh.exit(1);
    return
  }

  generateSealedSecretsManifests();
}

/* 
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
const SEALED_SECRETS_DIR_FOR_ENV = `${SEALED_SECRETS_BASE_DIR}/${ARGV.e}`;
async function generateSealedSecretsManifests() {
  const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = path.join(
    MANIFESTS_DIR,
    "generated",
    ARGV.e,
    "/**/**/**secret-*ml"
  );

  const unsealedSecretsFilePathsForEnv = await globAsync(UNSEALED_SECRETS_MANIFESTS_FOR_ENV, {
    dot: true,
  });

  unsealedSecretsFilePathsForEnv.forEach((unsealedSecretPath) => {
    if (ARGV.gss) {
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

    const shouldKeepSecretOutputs = ARGV.kuso;
    if (!shouldKeepSecretOutputs) {
      sh.rm("-rf", unsealedSecretPath);
    }

    const shouldRemoveSecretInPuts = ARGV.kusi;
    if (!shouldRemoveSecretInPuts) {
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

generateManifests();
if (ARGV.gss) {
  /* 
       This requires the above step with initial cluster setup making sure that the sealed secret controller is
       running in the cluster */
  doInitialClusterSetup();

  generateSealedSecretsManifests();
  // TODO: could conditionally check the installation of argocd also cos it may not be necessary for local dev
  sh.exec(`kubectl apply -f ${manifestsDirForEnv}/argocd/0-crd`);
  sh.exec(`kubectl apply -f ${manifestsDirForEnv}/argocd/1-manifest`);
  sh.exec(`kubectl apply -f ${SEALED_SECRETS_DIR_FOR_ENV}`);
}

#!/usr/bin/env node

import path from "path";
import glob from "glob";
import prompt from "prompt";
import sh from "shelljs";
import util from "util";
import yargs from "yargs/yargs";
import c from "chalk";

import { Environment } from "./../resources/shared/types/own-types";
import { environmentVariables } from "../resources/shared/validations";

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
      type: "boolean",
      // default: false,
      describe:
        "Generate sealed secrets manifests from generated plain secrets manifests",
      // demandOption: true,
    },
    kus: {
      alias: "keep-unsealed-secrets",
      type: "boolean",
      // default: false,
      describe:
        "Keep the original unsealed secrets manifests from generated plain secrets manifests",
      // demandOption: true,
    },
  })
  .parseSync();

prompt.override = ARGV;
prompt.start();

async function generateManifests() {
  const promptArgument: Pick<typeof ARGV, "gss"> = (await promptGetAsync([
    {
      name: "gss",
      required: true,
      type: "boolean",
      description: c.blueBright(
        "Generate sealed secrets from plain secrets manifests. (t/f)"
      ),
    },
  ])) as Pick<typeof ARGV, "gss">;

  // sh.cd(__dirname);
  // sh.exec("npm i");
  sh.exec("rm -rf ./login");
  sh.exec("npm i");
  sh.exec("mkdir ./login");
  sh.exec("pulumi login file://login");
  sh.exec(
    "export PULUMI_CONFIG_PASSPHRASE='' && pulumi stack init --stack dev"
  );

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
  sh.echo(c.greenBright(shGenerateManifestsOutput.stdout));
  sh.echo(c.redBright(shGenerateManifestsOutput.stderr));

  generateSealedSecretsManifests(promptArgument);
}

generateManifests();

async function generateSealedSecretsManifests(
  promptArgument: Pick<typeof ARGV, "gss">
) {
  const SEALED_SECRETS_BASE_DIR = path.join(MANIFESTS_DIR, "sealed-secrets");
  const SEALED_SECRETS_DIR = `${SEALED_SECRETS_BASE_DIR}/${ARGV.e}`;

  const UNSEALED_SECRETS_MANIFESTS_FOR_ENV = path.join(
    MANIFESTS_DIR,
    "generated",
    ARGV.e,
    "/**/**/**secret-*ml"
  );

  const unsealedSecretsFilePathsForEnv = await globAsync(
    UNSEALED_SECRETS_MANIFESTS_FOR_ENV,
    {
      dot: true,
    }
  );

  unsealedSecretsFilePathsForEnv.forEach((unsealedSecretPath) => {
    if (ARGV.gss || promptArgument.gss) {
      sh.echo(
        c.blueBright("Generating sealed secret from", unsealedSecretPath)
      );

      const secretName = path.basename(unsealedSecretPath);
      const sealedSecretFilePath = `${SEALED_SECRETS_DIR}/${secretName}`;

      sh.echo(c.greenBright(`Create ${SEALED_SECRETS_DIR} if not exists`));
      sh.mkdir("-p", SEALED_SECRETS_DIR);

      const kubeSeal = sh.exec(
        `kubeseal <${unsealedSecretPath} -o yaml >${sealedSecretFilePath}`,
        // { silent: true }
      );

      sh.echo(c.greenBright(kubeSeal.stdout));
      sh.echo(c.redBright(kubeSeal.stderr));

      sh.echo(
        c.greenBright(
          "Successfully generated sealed secret at",
          unsealedSecretPath
        )
      );
    }

    sh.echo(
      c.blueBright(
        `Removing unsealed plain secret manifest ${unsealedSecretPath}`
      )
    );
    sh.rm("-rf", unsealedSecretPath);
  });
}

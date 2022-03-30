#!/usr/bin/env node

import { Environment } from "./../resources/shared/types/own-types";
import sh from "shelljs";
import util from "util";
import { environmentVariables } from "../resources/shared/validations";
import yargs from "yargs/yargs";

const argv = yargs(process.argv.slice(2))
  .options({
    imt: { type: "string", alias: "image_tag", demandOption: true },
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
  })
  .parseSync();

type EnvName = keyof typeof environmentVariables;

export const IMAGE_TAG_REACT_WEB: EnvName = "IMAGE_TAG_REACT_WEB";
export const IMAGE_TAG_GRAPHQL_MONGO: EnvName = "IMAGE_TAG_GRAPHQL_MONGO";
export const IMAGE_TAG_GRPC_MONGO: EnvName = "IMAGE_TAG_GRPC_MONGO";
export const IMAGE_TAG_GRAPHQL_POSTGRES: EnvName = "IMAGE_TAG_GRAPHQL_POSTGRES";
export const ENVIRONMENT: EnvName = "ENVIRONMENT";
export const TEMPORARY_DIR: EnvName = "TEMPORARY_DIR";

function generateYamls() {
  const TAG = argv.imt;
  const ENVIRONMENT = argv.e;

  sh.cd(__dirname);
  sh.exec("npm i");
  sh.exec("rm -rf ./login");
  sh.exec("npm i");
  sh.exec("mkdir ./login");
  sh.exec("pulumi login file://login");
  sh.exec(
    "export PULUMI_CONFIG_PASSPHRASE='' && pulumi stack init --stack dev"
  );

  // image tag. All apps share same tag for now
  // Try to set this directly in process.env e.g
  // process.env.IMAGE_TAG_GRAPHQL_MONGO = 44

  sh.exec(
    `
      export ${IMAGE_TAG_REACT_WEB}="${TAG}" && \
      export ${IMAGE_TAG_GRAPHQL_MONGO}="${TAG}" && \
      export ${IMAGE_TAG_GRPC_MONGO}="${TAG}" && \
      export ${IMAGE_TAG_GRAPHQL_POSTGRES}="${TAG}" && \
      export ${ENVIRONMENT}="${ENVIRONMENT}" && \
      export PULUMI_CONFIG_PASSPHRASE="" && \
      pulumi update --yes --skip-preview --stack dev
      `
  );
}

generateYamls();

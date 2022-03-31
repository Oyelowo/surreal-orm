#!/usr/bin/env node

import { Environment } from "./../resources/shared/types/own-types";
import sh from "shelljs";
import util from "util";
import { environmentVariables } from "../resources/shared/validations";
import yargs from "yargs/yargs";

// import { execSync } from 'child_process';  // replace ^ if using ES modules

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

// var argv = require("yargs/yargs")(process.argv.slice(2))
//   .alias("i", "ingredient")
//   .describe("i", "choose your sandwich ingredients")
//   .choices("i", ["peanut-butter", "jelly", "banana", "pickles"])
//   .help("help").argv;

type EnvName = keyof typeof environmentVariables;
// export const envVarNames: Record<EnvName, EnvName> = {
//   IMAGE_TAG_REACT_WEB: "IMAGE_TAG_REACT_WEB",
//   IMAGE_TAG_GRAPHQL_MONGO: "IMAGE_TAG_GRAPHQL_MONGO",
//   IMAGE_TAG_GRPC_MONGO: "IMAGE_TAG_GRPC_MONGO",
//   IMAGE_TAG_GRAPHQL_POSTGRES: "IMAGE_TAG_GRAPHQL_POSTGRES",
//   ENVIRONMENT: "ENVIRONMENT",
//   TEMPORARY_DIR: "TEMPORARY_DIR",
// };
export const IMAGE_TAG_REACT_WEB: EnvName = "IMAGE_TAG_REACT_WEB";
export const IMAGE_TAG_GRAPHQL_MONGO: EnvName = "IMAGE_TAG_GRAPHQL_MONGO";
export const IMAGE_TAG_GRPC_MONGO: EnvName = "IMAGE_TAG_GRPC_MONGO";
export const IMAGE_TAG_GRAPHQL_POSTGRES: EnvName = "IMAGE_TAG_GRAPHQL_POSTGRES";
export const ENVIRONMENT: EnvName = "ENVIRONMENT";
export const TEMPORARY_DIR: EnvName = "TEMPORARY_DIR";

const p = __dirname;
// sh(`cd ${p}`)
// environmentVariables;

/* 
export ${IMAGE_TAG_REACT_WEB}="local" && \
export ${IMAGE_TAG_GRAPHQL_MONGO}="local" && \
export ${IMAGE_TAG_GRPC_MONGO}="local" && \
export ${IMAGE_TAG_GRAPHQL_POSTGRES}="local" && \
export ${ENVIRONMENT}="local" && \
*/

type Options = {
  cmd: `generate_${Environment}`;
  environment: Environment;
  imageTag: string;
};

function generateYamls({ cmd, imageTag, environment }: Options) {
  const TAG = argv.imt;
  sh.cd(__dirname);
  // sh.exec("npm i")
  sh.exec("rm -rf ./login");
  sh.exec("npm i");
  sh.exec("mkdir ./login");
  sh.exec("pulumi login file://login");
  sh.exec(
    "export PULUMI_CONFIG_PASSPHRASE='' && pulumi stack init --stack dev"
  );

  // # $1 -> image tag. All apps share same tag for now

  // Try to set this directly in process.env e.g
  // process.env.IMAGE_TAG_GRAPHQL_MONGO = 44
  sh.exec(
    `
      export ${IMAGE_TAG_REACT_WEB}="${TAG}" && \
      export ${IMAGE_TAG_GRAPHQL_MONGO}="${TAG}" && \
      export ${IMAGE_TAG_GRPC_MONGO}="${TAG}" && \
      export ${IMAGE_TAG_GRAPHQL_POSTGRES}="${TAG}" && \
      export ${ENVIRONMENT}="${environment}" && \
      export PULUMI_CONFIG_PASSPHRASE="" && \
      pulumi update --yes --skip-preview --stack dev
      `
  );
}

/* 
Example usage: 
npx ts-node scripts/script.ts generate_local <image-tag>
npx ts-node <this-file-name>.ts generate_local <image-tag>
*/
// const cmdName = z
//   .union([z.literal("generate_local"), z.literal("generate_production")])
//   .parse(process.argv.at(2));
// if (cmdName === "generate_local") {
//   generateYamls({
//     cmd: cmdName,
//     imageTag: process.argv.at(3)!,
//     environment: "local",
//   });
// }
// const cmdName = z
//   .union([z.literal("generate_local"), z.literal("generate_production")])
//   .parse(process.argv.at(2));
// if (cmdName === "generate_local") {
//   generateYamls({
//     cmd: cmdName,
//     imageTag: process.argv.at(3)!,
//     environment: "local",
//   });
// }

// if (cmdName === "generate_production") {
//   generateYamls({
//     cmd: cmdName,
//     imageTag: process.argv.at(3)!,
//     environment: "production",
//   });
// }

import path from "path";
import fs from "fs";
import glob from "glob";

function sealSecrets(environment: Environment | "temporary") {
  //   BASEDIR=./manifests/${ENVIRONMENT}
  // GENERATED_DIR=${BASEDIR}/generated-temporary
  // SECRETS_ENCRYPTED_DIR=${BASEDIR}/sealed-secrets
  const baseDir = path.join(__dirname, "..");
  const manifestsDir = path.join(baseDir, "manifests");
  const environmentManifestsDir = path.join(manifestsDir, environment);
  const sealedSecretsBaseDir = path.join(manifestsDir, "sealed-secrets");

  const secretPathsForEnvironment = path.join(
    environmentManifestsDir,
    "/**/**/**secret-*ml"
  );
  const sealedSecretsEnvironmentDir = path.join(
    sealedSecretsBaseDir,
    environment
  );
  //   shh(`
  // seal:
  // 	# Example usage: ENVIRONMENT="production" make seal

  // 	# Apply namespace first
  // 	kubectl apply -R -f ${environmentManifestsDir}/generated/namespaces

  // 	# Apply setups with sealed secret controller
  // 	kubectl apply -R -f  ${environmentManifestsDir}/generated/cluster-setup

  // 	# Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
  // 	kubectl rollout status deployment/sealed-secrets-controller -n=kube-system

  // 	mkdir -p ${sealedSecretsBaseDir}
  //   `);

  // sealSecretsInManifestsDir();
  glob(secretPathsForEnvironment, { dot: true }, (err, res) => {
    res.forEach((file) => {
      console.log("erer", res);
      shh(`
      kubeseal <${file} -o yaml >${sealedSecretsEnvironmentDir}/${path.basename(
        file
      )};
      echo "REMOVING unsealed secret ${file}"; 
      `);
      // rm -rf ${file};
    });
  });
}

sealSecrets("production");

// function sealSecretsInManifestsDir() {
//   const baseDir = path.join(__dirname, "..");
//   const manifestsDir = path.join(baseDir, "manifests");
//   const environmentManifestsDir = path.join(manifestsDir, "production");
//   const secretPathsForEnvironment = path.join(
//     environmentManifestsDir,
//     "/**/**secret-*ml"
//   );
//   const sealedSecretsDir = path.join(
//     manifestsDir,
//     "sealed-secrets",
//     "production"
//   );
//   // glob(src + "/**/*", callback);
//   glob(secretPathsForEnvironment, { dot: true }, (err, res) => {
//     res.forEach((file) => {
//       console.log("erer", res);
//       shh(`
//       mkdir -p ${sealedSecretsDir}
//       kubeseal <${file} -o yaml >${sealedSecretsDir}/${path.basename(file)};
//       echo "REMOVING unsealed secret ${file}";
//       rm -rf ${file};
//       `);
//     });
//   });
// }

// console.log("ererer", mn);
// shh(`
// 	@for FILE in ${environmentManifestsDir}/**/**/secret-*ml; do \
// 		kubeseal <$${FILE} -o yaml >${sealedSecretsDir}/$${FILE##*/}; \
// 		echo "REMOVING unsealed secret $${FILE}"; \
// 		rm -rf $${FILE}; \
// 	done

// 	rm -rf ${environmentManifestsDir}
// `)

// if (process.argv.at(2) === "generate_production") {
//   generateYamls({ cmd: "production", imageTag: process.argv.at(3)! });
// }
// generateYamls("production", process.argv.at(3)!)
// generateYamls("development", process.argv.at(3)!)
// generateYamls("staging", process.argv.at(3)!)

// console.log("Output was:\n", output);
// process.argv.at(2);

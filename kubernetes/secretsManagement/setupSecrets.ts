/* 
ADD INSTRUCTION HERE
 */

import * as z from "zod";

export type Secrets = z.infer<typeof secretsSchema>;

import fs from "fs";
import path from "path";
import { Environment } from "../resources/shared/types/own-types";
import { secretsLocalSample, secretsSample } from "./secretsSample";
import c from "chalk";
const ENVIRONMENTS: Environment[] = ["local", "development", "staging", "production"];
const secretType = "Secrets" as const;
const scriptName = path.basename(__filename).slice(0, -3);
const SECRET_UNSEALED_DIRECTORY_NAME = "secrets-unsealed" as const;
const UNSEALED_SECRETS_DIR = `${__dirname}/secrets-unsealed` as const;

type SecretUnseatFilePath = `${typeof UNSEALED_SECRETS_DIR}/${Environment}.ts`;

export function setupUnsealedSecretFiles() {
  fs.mkdir(UNSEALED_SECRETS_DIR, (err) => {
    // TODO: this should not be an error
    console.info(`Unsealed secrets directory already created`);
    // console.info(`Something went wrong creating unsealed secrets directory: Error: ${err}`);
  });

  ENVIRONMENTS.forEach((env) => {
    createSecretsConfigFile(env, false);
  });
}

export function clearUnsealedInputTsSecretFilesContents() {
  ENVIRONMENTS.forEach((env) => {
    createSecretsConfigFile(env, true);
  });
}
//

// HELPERS
const secretsSchema = z.object({
  "graphql-mongo": z.object({
    MONGODB_USERNAME: z.string().nonempty(),
    MONGODB_PASSWORD: z.string().nonempty(),
    MONGODB_ROOT_USERNAME: z.string().nonempty(),
    MONGODB_ROOT_PASSWORD: z.string().nonempty(),
    REDIS_USERNAME: z.string().nonempty(),
    REDIS_PASSWORD: z.string().nonempty(),
  }),
  "grpc-mongo": z.object({
    MONGODB_USERNAME: z.string().nonempty(),
    MONGODB_PASSWORD: z.string().nonempty(),
    MONGODB_ROOT_USERNAME: z.string().nonempty(),
    MONGODB_ROOT_PASSWORD: z.string().nonempty(),
  }),
  "graphql-postgres": z.object({
    POSTGRES_USERNAME: z.string().nonempty(),
    POSTGRES_PASSWORD: z.string().nonempty(),
  }),
  "react-web": z.object({
    GITHUB_CLIENT_ID: z.string().nonempty(),
    GITHUB_CLIENT_SECRET: z.string().nonempty(),
    GOOGLE_CLIENT_ID: z.string().nonempty(),
    GOOGLE_CLIENT_SECRET: z.string().nonempty(),
  }),
  argocd: z.object({
    ADMIN_PASSWORD: z.string().nonempty(),
    type: z.literal("git"),
    url: z.literal("https://github.com/Oyelowo/modern-distributed-app-template"),
    username: z.literal("Oyelowo"),
    password: z.string().nonempty(),
  }),
});

function getFilePath(environment: Environment): SecretUnseatFilePath {
  return `${UNSEALED_SECRETS_DIR}/${environment}.ts`;
}

// import chalk from "chalk";

async function createSecretsConfigFile(environment: Environment, resetConfigs: boolean) {
  const filePath = getFilePath(environment);

  const content = `
    import {${secretType}} from "../${scriptName}"
     export const SECRET_${environment.toUpperCase()}: ${secretType} = ${JSON.stringify(
    environment === "local" ? secretsLocalSample : secretsSample
  )};
    `;

  fs.writeFile(filePath, content, { flag: resetConfigs ? "wx" : "" }, (error) => {
    // if (err) throw err;
    if (error) {
      console.info("File already created previously!");
    }
    console.info(
      c.blueBright(`Secret files created and gitignored at ${filePath}! NEVER commit these secrets to git!🎉`)
    );
  });
}

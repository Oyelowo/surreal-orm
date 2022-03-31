/* 
ADD INSTRUCTION HERE
 */

import * as z from "zod";

export type Secrets = z.infer<typeof secretsSchema>;

import fs from "fs";
import path from "path";
import { Environment } from "../resources/shared/types/own-types";
import { secretsSample } from "./secretsSample";

const ENVIRONMENTS: Environment[] = [
  "local",
  "development",
  "staging",
  "production",
];
const secretType = "Secrets" as const;
const scriptName = path.basename(__filename).slice(0, -3);
const SECRET_UNSEALED_DIRECTORY_NAME = "secrets-unsealed" as const;
const UNSEALED_SECRETS_DIR = `${__dirname}/secrets-unsealed` as const;

type SecretUnseatFilePath = `${typeof UNSEALED_SECRETS_DIR}/${Environment}.ts`;

export function setupUnsealedSecretFiles() {
  fs.mkdir(UNSEALED_SECRETS_DIR, (err) => {
    console.info(
      `Something went wrong creating unsealed secrets directory: Error: ${err}`
    );
  });
  createGitIgnoreFile();
  ENVIRONMENTS.forEach((env) => {
    createSecretsConfigFile(env, false);
  });
}

export function clearUnsealedSecretFilesContents() {
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
    REDIS_USERNAME: z.string().nonempty(),
    REDIS_PASSWORD: z.string().nonempty(),
  }),
  "grpc-mongo": z.object({
    MONGODB_USERNAME: z.string().nonempty(),
    MONGODB_PASSWORD: z.string().nonempty(),
  }),
  "graphql-postgres": z.object({
    POSTGRES_USERNAME: z.string().nonempty(),
    POSTGRES_PASSWORD: z.string().nonempty(),
  }),
  "react-web": z.object({
    GITHUB_ID: z.string().nonempty(),
    GITHUB_SECRET: z.string().nonempty(),
    GOOGLE_ID: z.string().nonempty(),
    GOOGLE_SECRET: z.string().nonempty(),
  }),
  argocd: z.object({
    ADMIN_PASSWORD: z.string().nonempty(),
  }),
});

function getFilePath(environment: Environment): SecretUnseatFilePath {
  return `${UNSEALED_SECRETS_DIR}/${environment}.ts`;
}

// import chalk from "chalk";

async function createSecretsConfigFile(
  environment: Environment,
  overwrite: boolean
) {
  const filePath = getFilePath(environment);

  const content = `
    import {${secretType}} from "../${scriptName}"
     export const SECRET_${environment.toUpperCase()}: ${secretType} = ${JSON.stringify(
    secretsSample
  )};
    `;

  fs.writeFile(filePath, content, { flag: overwrite ? "" : "wx" }, (error) => {
    // if (err) throw err;
    if (error) {
      console.warn("File already created!🎉");
    }
    console.warn(
      `Secret files created and gitignored!!! 🎉  
          Make sure you never push these secrets to git!🎉`
    );
  });
}

async function createGitIgnoreFile() {
  const filePath = `${__dirname}/.gitignore`;
  const content = SECRET_UNSEALED_DIRECTORY_NAME;

  fs.writeFile(filePath, content, { flag: "" as "" | "ws" }, async (err) => {
    // if (err) throw err;
    if (err) {
      console.warn("err", err);
    }
    console.log("It's saved!");
  });
}

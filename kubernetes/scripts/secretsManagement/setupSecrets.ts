/* 
ADD INSTRUCTION HERE
 */
import * as z from "zod";

import fs from "fs";
import path from "path";
import c from "chalk";

import { Environment } from "../../resources/shared/types/own-types";
import { getUnsealedSecretsConfigFilesBaseDir } from "./../../resources/shared/manifestsDirectory";
import { secretsLocalSample, secretsSample } from "./secretsSample";

const ENVIRONMENTS: Environment[] = ["local", "development", "staging", "production"];
const UNSEALED_SECRETS_DIR = getUnsealedSecretsConfigFilesBaseDir();
export type Secrets = z.infer<typeof secretsSchema>;
const SECRETS_TYPE = "Secrets" as const; // This should be same as the secrets type above

type SecretUnseatFilePath = `${typeof UNSEALED_SECRETS_DIR}/${Environment}.ts`;

export function setupUnsealedSecretFiles() {
  fs.mkdir(UNSEALED_SECRETS_DIR, (err) => {
    console.info(`Unsealed secrets directory already created`);
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

async function createSecretsConfigFile(environment: Environment, resetConfigs: boolean) {
  const filePath = getFilePath(environment);
  const thisFileRelativeDir = __dirname.split("/").slice(-2).join("/");
  const thisFileName = path.basename(__filename).slice(0, -3);
  // This is for each of the secret files
  const content = `
    import {${SECRETS_TYPE}} from "../${thisFileRelativeDir}/${thisFileName}";
    
     export const SECRET_${environment.toUpperCase()}: ${SECRETS_TYPE} = ${JSON.stringify(
    environment === "local" ? secretsLocalSample : secretsSample
  )};
    `;

  fs.readFile(filePath, (error) => {
    if (error) {
      // Only create new config file if not already exists
      writeSecretConfigFile(filePath, content);
    }
  });

  if (resetConfigs) {
    // Override existing secret config file regardless if specified
    writeSecretConfigFile(filePath, content);
  }
}

function writeSecretConfigFile(filePath: string, content: string) {
  fs.writeFile(filePath, content, { flag: "wx" }, (error) => {
    if (error) {
      console.info("File already created previously!");
    }
    console.info(
      c.blueBright(`Secret files created and gitignored at ${filePath}! NEVER commit these secrets to git!🎉`)
    );
  });
}

import { ResourceName } from './../../resources/shared/types/own-types';
/* 
ADD INSTRUCTION HERE
 */
import * as z from "zod";
import sh from "shelljs";

import fs from "fs";
import path from "path";
import c from "chalk";

import { Environment } from "../../resources/shared/types/own-types";
import { getPlainSecretsConfigFilesBaseDir } from "./../../resources/shared/manifestsDirectory";
import { secretsLocalSample, secretsSample } from "./secretsSample";

const ENVIRONMENTS: Environment[] = ["local", "development", "staging", "production"];
const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();
// export type Secrets = z.infer<typeof secretsSchema>;

const SECRETS_TYPE = "Secrets" as const; // This should be same as the secrets type above

type SecretUnseatFilePath = `${typeof PLAIN_SECRETS_CONFIGS_DIR}/${Environment}.ts`;

export function setupPlainSecretTSFiles() {
  fs.mkdir(PLAIN_SECRETS_CONFIGS_DIR, (err) => {
    console.info(`Unsealed secrets directory already created`);
  });

  ENVIRONMENTS.forEach((env) => {
    createSecretsConfigFile(env, false);
  });
}

export function clearPlainInputTsSecretFilesContents() {
  ENVIRONMENTS.forEach((env) => {
    createSecretsConfigFile(env, true);
  });
}
//

// HELPERS
type SecretsSchema = {
  "graphql-mongo": {
    MONGODB_USERNAME: string;
    MONGODB_PASSWORD: string;
    MONGODB_ROOT_USERNAME: string;
    MONGODB_ROOT_PASSWORD: string;
    REDIS_USERNAME: string;
    REDIS_PASSWORD: string;
  },
  "grpc-mongo": {
    MONGODB_USERNAME: string;
    MONGODB_PASSWORD: string;
    MONGODB_ROOT_USERNAME: string;
    MONGODB_ROOT_PASSWORD: string;
  },
  "graphql-postgres": {
    POSTGRES_USERNAME: string;
    POSTGRES_PASSWORD: string;
  },
  "react-web": {
    GITHUB_CLIENT_ID: string;
    GITHUB_CLIENT_SECRET: string;
    GOOGLE_CLIENT_ID: string;
    GOOGLE_CLIENT_SECRET: string;
  },
  argocd: {
    ADMIN_PASSWORD: string;
    type: "git",
    url: "https://github.com/Oyelowo/modern-distributed-app-template",
    username: "Oyelowo",
    password: string,
  },
};

export type Secrets = Partial<Record<ResourceName, {}>> & SecretsSchema;

function getFilePath(environment: Environment): SecretUnseatFilePath {
  return `${PLAIN_SECRETS_CONFIGS_DIR}/${environment}.ts`;
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
    sh.rm("-rf", filePath);
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

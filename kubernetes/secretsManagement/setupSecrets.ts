import * as z from "zod";

// interface Secrets {
//   "graphql-mongo": {
//     MONGODB_USERNAME: string;
//     MONGODB_PASSWORD: string;
//     REDIS_USERNAME: string;
//     REDIS_PASSWORD: string;
//   };
//   "grpc-mongo": {
//     MONGODB_USERNAME: string;
//     MONGODB_PASSWORD: string;
//   };
//   "graphql-postgres": {
//     POSTGRES_USERNAME: string;
//     POSTGRES_PASSWORD: string;
//   };
//   "react-web": {
//     GITHUB_ID: string;
//     GITHUB_SECRET: string;
//     GOOGLE_ID: string;
//     GOOGLE_SECRET: string;
//   };
//   argocd: {
//     ADMIN_PASSWORD: string;
//   };
// }

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

export type Secrets = z.infer<typeof secretsSchema>;

// NOTE: I initially was encoding the secrets in base64 but it turns out
// that bitnami sealed secrets not only handles encryption but base64 encoding of the
// secrets before encrypting them
const secretsSample: Secrets = {
  "graphql-mongo": {
    MONGODB_USERNAME: "",
    MONGODB_PASSWORD: "",
    REDIS_USERNAME: "",
    REDIS_PASSWORD: "",
  },
  "grpc-mongo": {
    MONGODB_USERNAME: "",
    MONGODB_PASSWORD: "",
  },
  "graphql-postgres": {
    POSTGRES_USERNAME: "",
    POSTGRES_PASSWORD: "",
  },
  "react-web": {
    GITHUB_ID: "",
    GITHUB_SECRET: "",
    GOOGLE_ID: "",
    GOOGLE_SECRET: "",
  },
  argocd: {
    ADMIN_PASSWORD: "",
  },
} as const;

import fs from "fs";
import path from "path";
import { Environment } from "../resources/shared/types/own-types";

// import { secretsLocalEnvironment } from "./local";

// const filePath = "./local.ts";
const secretType = "Secrets";
const scriptName = path.basename(__filename).slice(0, -3);

function getFilePath(
  environment: Environment
): `./secrets-unsealed-${Environment}.ts` {
  return `./secrets-unsealed-${environment}.ts`;
}

function getFileName(
  environment: Environment
): `secrets-unsealed-${Environment}.ts` {
  const path = getFilePath(environment);
  return path.split("/")[1] as `secrets-unsealed-${Environment}.ts`;
}
// import chalk from "chalk";

async function createSecretsConfigFile(
  environment: Environment,
  overwrite: boolean
) {
  const filePath = getFilePath(environment);
  const content = `
    import {${secretType}} from "./${scriptName}"
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
const ENVIRONMENTS: Environment[] = [
  "local",
  "development",
  "staging",
  "production",
];

function createSecretFiles() {
  ENVIRONMENTS.forEach((env) => {
    createSecretsConfigFile(env, false);
  });
}

createSecretFiles();

export function clearSecretFilesContents() {
  ENVIRONMENTS.forEach((env) => {
    createSecretsConfigFile(env, true);
  });
}
// clearSecretFilesContents()

async function createGitIgnoreFile(filePath: string) {
  const filePaths = ENVIRONMENTS.map(getFileName);
  const content = filePaths.map((p) => `${p}\n`).join("");

  fs.writeFile(filePath, content, { flag: "wx" }, async (err) => {
    // if (err) throw err;
    if (err) {
      console.warn("err", err);
    }
    console.log("It's saved!");
  });
}
createGitIgnoreFile(".gitignore");

// const secretRecord: Record<Environment, typeof secretsJonFileExample> = {
//   production: secretsJonFile,
//   development: secretsJonFile,
//   staging: secretsJonFile,
//   local: secretsJonFileExample,
// };

// const secretJson = secretRecord[environmentVariables.ENVIRONMENT];

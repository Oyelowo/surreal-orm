import { Secrets } from "./setupSecrets";

// NOTE: I initially was encoding the secrets in base64 but it turns out
// that bitnami sealed secrets not only handles encryption but base64 encoding of the
// secrets before encrypting them
export const secretsSample: Secrets = {
  "graphql-mongo": {
    MONGODB_USERNAME: "",
    MONGODB_PASSWORD: "",
    MONGODB_ROOT_USERNAME: "",
    MONGODB_ROOT_PASSWORD: "",
    REDIS_USERNAME: "",
    REDIS_PASSWORD: "",
  },
  "grpc-mongo": {
    MONGODB_USERNAME: "",
    MONGODB_PASSWORD: "",
    MONGODB_ROOT_USERNAME: "",
    MONGODB_ROOT_PASSWORD: "",
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
    type: "git",
    url: "https://github.com/Oyelowo/modern-distributed-app-template",
    username: "Oyelowo",
    password: "my-password-or-personal-access-token",
  },
} as const;

export const secretsLocalSample: Secrets = {
  "graphql-mongo": {
    MONGODB_USERNAME: "username",
    MONGODB_PASSWORD: "password",
    MONGODB_ROOT_USERNAME: "root_username",
    MONGODB_ROOT_PASSWORD: "root_password",
    REDIS_USERNAME: "username",
    REDIS_PASSWORD: "password",
  },
  "grpc-mongo": {
    MONGODB_USERNAME: "username",
    MONGODB_PASSWORD: "password",
    MONGODB_ROOT_USERNAME: "root_username",
    MONGODB_ROOT_PASSWORD: "root_password",
  },
  "graphql-postgres": {
    POSTGRES_USERNAME: "username",
    POSTGRES_PASSWORD: "password",
  },
  "react-web": {
    GITHUB_ID: "",
    GITHUB_SECRET: "",
    GOOGLE_ID: "",
    GOOGLE_SECRET: "",
  },
  argocd: {
    ADMIN_PASSWORD: "12345",
    type: "git",
    url: "https://github.com/Oyelowo/modern-distributed-app-template",
    username: "Oyelowo",
    password: "my-password-or-personal-access-token",
  },
} as const;

import { Secrets } from "./setupSecrets";

// NOTE: I initially was encoding the secrets in base64 but it turns out
// that bitnami sealed secrets not only handles encryption but base64 encoding of the
// secrets before encrypting them
export const secretsSample: Secrets = {
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

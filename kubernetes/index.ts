// export * from "./resources/shared/namespaces";
// export * from "./secretsManagement/setupSecrets";

import { setupUnsealedSecretFiles } from "./secretsManagement/setupSecrets";
setupUnsealedSecretFiles();

// export * from "./resources/shared/cluster";
export * from "./resources/shared";

// Ingress controller and ingress rule
export * from "./resources/ingress";

export * from "./resources/secrets";

export * from "./resources/argocd";

// Rust server backend with support for graphql, mongodb and postgres
// RUST WORKSPACE APPS
export * from "./resources/graphql-mongo";
// Uncomment these if you want to work with graphql and postgres or grpc with mongodb
// export * from "./resources/graphql-postgres";
// export * from "./resources/grpc-mongo";

// TYPESCRIPT WORKSPACE APPS
// Web app. Nextjs with client and server support. Server is at /api
export * from "./resources/react-web";

// export * from "./resources";

// Turn this on if you want secret files values to be empty e.g within ./secretsManagement/secrets-unsealed-development.ts
// clearSecretFilesContents()

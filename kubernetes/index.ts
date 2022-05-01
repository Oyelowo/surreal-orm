// export * from "./resources/shared/namespaces";
// export * from "./secretsManagement/setupSecrets";

// import { setupUnsealedSecretFiles } from "./secretsManagement/setupSecrets";
// setupUnsealedSecretFiles();

// export * from "./resources/shared/cluster";


// Ingress controller and ingress rule
export * from "./resources/namespaces";
export * from "./resources/infrastructure/ingress-controller";
export * from "./resources/infrastructure/secrets";
export * from "./resources/infrastructure/argocd";
export * from "./resources/infrastructure/cert-manager";
export * from "./resources/infrastructure/linkerd";
// export * from "./resources/infrastructure/linkerdViz";



// Rust server backend with support for graphql, mongodb and postgres
// RUST WORKSPACE APPS
export * from "./resources/services/graphql-mongo";
// Uncomment these if you want to work with graphql and postgres or grpc with mongodb
// export * from "./resources/applications/graphql-postgres";
// export * from "./resources/applications/grpc-mongo";

// TYPESCRIPT WORKSPACE APPS
// Web app. Nextjs with client and server support. Server is at /api
export * from "./resources/services/react-web";

// export * from "./resources";

// Turn this on if you want secret files values to be empty e.g within ./secretsManagement/secrets-unsealed-development.ts
// clearSecretFilesContents()

// INFRASTRUCTURE
export * from './src/infrastructure/namespaces/index.js';
export * from './src/infrastructure/argocd/index.js';
export * from './src/infrastructure/cert-manager/index.js';
export * from './src/infrastructure/ingress/index.js';
export * from './src/infrastructure/linkerd/index.js';
export * from './src/infrastructure/sealed-secrets/index.js';
// SERVICES
// Rust server backend with support for graphql, mongodb and postgres
// RUST WORKSPACE APPS
export * from './src/services/graphql-mongo/index.js';
// Uncomment these if you want to work with graphql and postgres or grpc with mongodb
// export * from "./resources/applications/graphql-postgres/index.js";
// export * from "./resources/applications/grpc-mongo/index.js";
// TYPESCRIPT WORKSPACE APPS
// Web app. Nextjs with client and server support. Server is at /api
export * from './src/services/react-web/index.js';

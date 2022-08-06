// INFRASTRUCTURE
export * from './resources/infrastructure/namespaces/index.js';
export * from './resources/infrastructure/argocd/index.js';
export * from './resources/infrastructure/cert-manager/index.js';
export * from './resources/infrastructure/ingress/index.js';
export * from './resources/infrastructure/linkerd/index.js';
export * from './resources/infrastructure/sealed-secrets/index.js';
// SERVICES
// Rust server backend with support for graphql, mongodb and postgres
// RUST WORKSPACE APPS
export * from './resources/services/graphql-mongo/index.js';
// Uncomment these if you want to work with graphql and postgres or grpc with mongodb
// export * from "./resources/applications/graphql-postgres/index.js";
// export * from "./resources/applications/grpc-mongo/index.js";
// TYPESCRIPT WORKSPACE APPS
// Web app. Nextjs with client and server support. Server is at /api
export * from './resources/services/react-web/index.js';

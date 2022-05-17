// INFRASTRUCTURE
export * from './resources/infrastructure/argocd'
export * from './resources/infrastructure/cert-manager'
export * from './resources/infrastructure/ingress'
export * from './resources/infrastructure/linkerd'
export * from './resources/infrastructure/sealed-secrets'
export * from './resources/namespaces'
// SERVICES
// Rust server backend with support for graphql, mongodb and postgres
// RUST WORKSPACE APPS
export * from './resources/services/graphql-mongo'
// Uncomment these if you want to work with graphql and postgres or grpc with mongodb
// export * from "./resources/applications/graphql-postgres";
// export * from "./resources/applications/grpc-mongo";
// TYPESCRIPT WORKSPACE APPS
// Web app. Nextjs with client and server support. Server is at /api
export * from './resources/services/react-web'





import { AppConfigs } from "../shared/types";

export const graphqlPostgresSettings: AppConfigs<
  "graphql-mongo",
  "postgresdb",
  "development"
> = {
  kubeConfig: {
    resourceName: "graphql-mongo",
    requestMemory: "70Mi",
    requestCpu: "100m",
    limitMemory: "200Mi",
    limitCpu: "100m",
    host: "0.0.0.0",
    image: "oyelowo/graphql-mongo",
  },

  envVars: {
    APP_ENVIRONMENT: "local",
    APP_HOST: "0.0.0.0",
    APP_PORT: "8000",
    POSTGRES_DATABASE_NAME: "graphql-mongo-database",
    POSTGRES_NAME: "graphql-mongo-database",
    POSTGRES_USERNAME: "postgres",
    POSTGRES_PASSWORD: "1234",
    POSTGRES_HOST: "graphql-mongo-database.development", // the name of the postgres service being connected to. The name has suffices(primary|read etc) if using replcated architecture
    POSTGRES_PORT: "5432",
    POSTGRES_SERVICE_NAME: "graphql-mongo-database",
  },
  metadata: {
    name: "graphql-mongo",
    namespace: "development",
  },
};

import { devNamespaceName } from "./../shared/namespaces";
import { Environemt, Settings } from "../shared/types";

export const graphqlPostgresSettings: Settings = {
  resourceName: "graphql-postgres",
  requestMemory: "1G",
  requestCpu: "100m",
  limitMemory: "1G",
  limitCpu: "100m",
  host: "0.0.0.0",
  image: "oyelowo/graphql-postgres",
} as const;

type AppEnvironmentVariables = {
  APP_ENVIRONMENT: Environemt;
  APP_HOST: "0.0.0.0" | string;
  APP_PORT: "8000" | `${number}`;

  POSTGRES_NAME: string;
  POSTGRES_DATABASE_NAME: string;
  POSTGRES_USERNAME: string;
  POSTGRES_PASSWORD: string;
  POSTGRES_HOST: string;
  POSTGRES_PORT: "5432";
  POSTGRES_SERVICE_NAME: `postgres-database`;
  // DATABASE_URL: postgres://postgres:1234@postgres-graphql-postgres:5432/my_db
};

export const graphqlPostgresEnvironmentVariables: AppEnvironmentVariables = {
  APP_ENVIRONMENT: "local",
  APP_HOST: "0.0.0.0",
  APP_PORT: "8000",
  POSTGRES_DATABASE_NAME: "db0",
  POSTGRES_NAME: "db0",
  POSTGRES_USERNAME: "postgres",
  POSTGRES_PASSWORD: "1234",
  POSTGRES_HOST: `postgres-database.${devNamespaceName}`,
  POSTGRES_PORT: "5432",
  POSTGRES_SERVICE_NAME: "postgres-database",
} as const;

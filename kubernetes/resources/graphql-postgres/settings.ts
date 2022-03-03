import { devNamespaceName } from "./../shared/namespaces";
import { Environemt, Settings } from "../shared/types";

export const graphqlPostgresSettings: Settings<'graphql-postgres'> = {
  resourceName: "graphql-postgres",
  requestMemory: "70Mi",
  requestCpu: "100m",
  limitMemory: "200Mi",
  limitCpu: "100m",
  host: "0.0.0.0",
  image: "oyelowo/graphql-postgres",
} as const;

type AppEnvVars = {
  APP_ENVIRONMENT: Environemt;
  APP_HOST: "0.0.0.0" | string;
  APP_PORT: "8000" | `${number}`;

  POSTGRES_NAME: string;
  POSTGRES_DATABASE_NAME: string;
  POSTGRES_USERNAME: string;
  POSTGRES_PASSWORD: string;
  POSTGRES_HOST: string;
  POSTGRES_PORT: "5432";
  POSTGRES_SERVICE_NAME: `graphql-postgres-database`;
  // DATABASE_URL: postgres://postgres:1234@postgres-graphql-postgres:5432/my_db
};

export const graphqlPostgresEnvVars: AppEnvVars = {
  APP_ENVIRONMENT: "local",
  APP_HOST: "0.0.0.0",
  APP_PORT: "8000",
  POSTGRES_DATABASE_NAME: "db0",
  POSTGRES_NAME: "db0",
  POSTGRES_USERNAME: "postgres",
  POSTGRES_PASSWORD: "1234",
  POSTGRES_HOST: `postgres-database.${devNamespaceName}`, // the name of the postgres service being connected to. The name has suffices(primary|read etc) if using replcated architecture
  POSTGRES_PORT: "5432",
  POSTGRES_SERVICE_NAME: "graphql-postgres-database",
} as const;

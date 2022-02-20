import { devNamespaceName } from "../shared/namespaces";
import { Environemt } from "../shared/types";

type AppEnvVars = {
  APP_ENVIRONMENT: Environemt;
  APP_HOST: "0.0.0.0" | string;
  APP_PORT: "50051" | `${number}`;
  POSTGRES_MONGO_HOST: string;
  POSTGRES_MONGO_PORT: "8000";
  POSTGRES_MONGO_SERVICE_NAME: string;
};

export const reactWebEnvVars: AppEnvVars = {
  APP_ENVIRONMENT: "local",
  APP_HOST: "0.0.0.0",
  APP_PORT: "3000",
  POSTGRES_MONGO_HOST: `react-web-database.${devNamespaceName}`,
  POSTGRES_MONGO_SERVICE_NAME: "raect-web-database",
  POSTGRES_MONGO_PORT: "8000",
} as const;

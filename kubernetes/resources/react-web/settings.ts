import { devNamespaceName } from "../shared/namespaces";
import { Environemt, Settings } from "../shared/types";

export const reactWebSettings: Settings = {
  resourceName: "react-web",
  requestMemory: "150Mi",
  requestCpu: "100m",
  limitMemory: "200Mi",
  limitCpu: "100m",
  host: "0.0.0.0",
  image: "oyelowo/react-web",
};


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

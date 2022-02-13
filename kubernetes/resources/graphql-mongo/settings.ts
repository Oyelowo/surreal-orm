import { Settings } from '../shared/types';

export const graphqlMongoSettings: Settings = {
  requestMemory: "1G",
  requestCpu: "100m",
  limitMemory: "1G",
  limitCpu: "100m",
  host: "0.0.0.0",
  image: "oyelowo/graphql-mongo",
};

type AppEnvironmentVariables = {
  APP_ENVIRONMENT: "local" | "development" | "staging" | "production";
  APP_HOST: "0.0.0.0" | string;
  APP_PORT: "8000" | `${number}`;
  MONGODB_NAME: string;
  MONGODB_USERNAME: string;
  MONGODB_PASSWORD: string;
  MONGODB_HOST: string;
  MONGODB_PORT: "27017";
};

export const graphqlMongoEnvironmentVariables: AppEnvironmentVariables = {
  APP_ENVIRONMENT: "local",
  APP_HOST: "0.0.0.0",
  APP_PORT: "8000",
  MONGODB_NAME: "mydb",
  MONGODB_USERNAME: "mongo",
  MONGODB_PASSWORD: "password",
  MONGODB_HOST: "mongodb-graphql",
  MONGODB_PORT: "27017",
};

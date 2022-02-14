import { Settings } from '../shared/types';

export const graphqlMongoSettings: Settings = {
  resourceName: "graphql-mongo",
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

 // `http://${name}.${namespace}:${port}`;

export const graphqlMongoEnvironmentVariables: AppEnvironmentVariables = {
  APP_ENVIRONMENT: "local",
  APP_HOST: "0.0.0.0",
  APP_PORT: "8000",
  MONGODB_NAME: "mydb",
  MONGODB_USERNAME: "root_user",
  MONGODB_PASSWORD: "root_password",
  MONGODB_HOST: "graphql-mongodb-headless.development",
  // MONGODB_HOST: "graphql-mongod-0.graphql-mongod-headless.development.svc.cluster.local",
  MONGODB_PORT: "27017",
};
// const graphqlMongoEnvironmentVariables: AppEnvironmentVariables = {
//   APP_ENVIRONMENT: "local",
//   APP_HOST: "0.0.0.0",
//   APP_PORT: "8000",
//   MONGODB_NAME: "mydb",
//   MONGODB_USERNAME: "mongo",
//   MONGODB_PASSWORD: "fakepassword",
//   MONGODB_HOST: "mongodb-graphql",
//   MONGODB_PORT: "27017",
// };

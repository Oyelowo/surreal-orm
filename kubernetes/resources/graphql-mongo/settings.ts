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

 /* 
 mongodb://username0@localhost:27017/?connectTimeoutMS=10000&authSource=db0&authMechanism=SCRAM-SHA-256&3t.uriVersion=3&3t.connection.name=db0&3t.alwaysShowAuthDB=true&3t.alwaysShowDBFromUserRole=true
 */

export const graphqlMongoEnvironmentVariables: AppEnvironmentVariables = {
  APP_ENVIRONMENT: "local",
  APP_HOST: "0.0.0.0",
  APP_PORT: "8000",
  MONGODB_NAME: "db0",
  MONGODB_USERNAME: "username0",
  MONGODB_PASSWORD: "password0",
  MONGODB_HOST: "mongo-graphql.development",
  // hostAndPort":"graphql-mongo-0.mongo-graphql.development.svc.cluster.local:27017
  // MONGODB_HOST: "graphql-mongod-0.graphql-mongod-headless.development.svc.cluster.local",
  // const url = 'mongodb://username1:$[password]@mongo-graphql.development:27017/db1?authSource=$[authSource]';

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

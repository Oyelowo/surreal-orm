import { AppConfigs } from "../shared/types";

export const graphqlMongoSettings: AppConfigs<
  "graphql-mongo",
  "mongodb",
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
    APP_ENVIRONMENT: "development",
    APP_HOST: "0.0.0.0",
    APP_PORT: "8000",
    MONGODB_NAME: "graphql-mongo-database",
    MONGODB_USERNAME: "username0",
    MONGODB_PASSWORD: "password0",
    MONGODB_HOST: "graphql-mongo-database.development",
    MONGODB_SERVICE_NAME: "graphql-mongo-database",
    // hostAndPort":"graphql-mongo-0.mongo-graphql.development.svc.cluster.local:27017
    // MONGODB_HOST: "graphql-mongod-0.graphql-mongod-headless.development.svc.cluster.local",
    // const url = 'mongodb://username1:$[password]@mongo-graphql.development:27017/db1?authSource=$[authSource]';

    MONGODB_PORT: "27017",
  },
};

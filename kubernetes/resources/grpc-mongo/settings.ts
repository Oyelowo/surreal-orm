import { AppConfigs } from "../shared/types";

export const grpcMongoSettings: AppConfigs<
  "grpc-mongo",
  "mongodb",
  "development"
> = {
  kubeConfig: {
    requestMemory: "70Mi",
    requestCpu: "100m",
    limitMemory: "200Mi",
    limitCpu: "100m",
    replicaCount: 3,
    host: "0.0.0.0",
    image: "oyelowo/grpc-mongo",
  },

  envVars: {
    APP_ENVIRONMENT: "development",
    APP_HOST: "0.0.0.0",
    APP_PORT: "50051",
    MONGODB_NAME: "grpc-mongo-database",
    MONGODB_USERNAME: "username0",
    MONGODB_PASSWORD: "password0",
    MONGODB_HOST: "grpc-mongo-database.development",
    MONGODB_SERVICE_NAME: "grpc-mongo-database",
    // hostAndPort":"grpc-mongo-0.mongo-graphql.development.svc.cluster.local:27017
    // MONGODB_HOST: "grpc-mongod-0.grpc-mongod-headless.development.svc.cluster.local",
    // const url = 'mongodb://username1:$[password]@mongo-grpc.development:27017/db1?authSource=$[authSource]';

    MONGODB_PORT: "27017",
  },
  metadata: {
    name: "grpc-mongo",
    namespace: "development",
  },
};

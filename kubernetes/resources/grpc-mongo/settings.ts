import { namespaceNames } from "./../shared/namespaces";
import { getEnvironmentVariables } from "../shared/validations";
import { AppConfigs } from "./../shared/types/own-types";
import { getSecretForApp } from "../../secretsManagement";

const environmentVariables = getEnvironmentVariables();
const environment = environmentVariables.ENVIRONMENT;
// TODO: Rethink this abstraction for secret
const secretsFromLocalConfigs = getSecretForApp("grpc-mongo");

export const grpcMongoSettings: AppConfigs<"grpc-mongo", "mongodb", "applications"> = {
  kubeConfig: {
    requestMemory: "70Mi",
    requestCpu: "100m",
    limitMemory: "200Mi",
    limitCpu: "100m",
    replicaCount: 3,
    host: "0.0.0.0",
    image: `ghcr.io/oyelowo/grpc-mongo:${environmentVariables.IMAGE_TAG_GRPC_MONGO}`,
  },

  envVars: {
    APP_ENVIRONMENT: environment,
    APP_HOST: "0.0.0.0",
    APP_PORT: "50051",
    MONGODB_NAME: "grpc-mongo-database",
    MONGODB_USERNAME: secretsFromLocalConfigs.MONGODB_USERNAME,
    MONGODB_PASSWORD: secretsFromLocalConfigs.MONGODB_PASSWORD,
    MONGODB_ROOT_USERNAME: secretsFromLocalConfigs.MONGODB_ROOT_USERNAME,
    MONGODB_ROOT_PASSWORD: secretsFromLocalConfigs.MONGODB_ROOT_PASSWORD,
    MONGODB_HOST: "grpc-mongo-database.applications",
    MONGODB_SERVICE_NAME: "grpc-mongo-database",
    MONGODB_STORAGE_CLASS: "linode-block-storage-retain",
    // hostAndPort":"grpc-mongo-0.mongo-graphql.development.svc.cluster.local:27017
    // MONGODB_HOST: "grpc-mongod-0.grpc-mongod-headless.development.svc.cluster.local",
    // const url = 'mongodb://username1:$[password]@mongo-grpc.development:27017/db1?authSource=$[authSource]';

    MONGODB_PORT: "27017",
  },
  metadata: {
    name: "grpc-mongo",
    namespace: namespaceNames.applications,
  },
};

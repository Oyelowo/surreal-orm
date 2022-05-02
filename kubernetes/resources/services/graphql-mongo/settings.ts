import { namespaceNames } from "../../namespaces/util";
import { getEnvironmentVariables } from "../../shared/validations";
import { AppConfigs } from "../../shared/types/own-types";
import { getSecretsForApp } from "../../../scripts/secretsManagement/getSecretsForApp";

const environment = getEnvironmentVariables().ENVIRONMENT;
const secretsFromLocalConfigs = getSecretsForApp("graphql-mongo");

// TODO: ADD A NEW KEY - SECRETS TO THE config which would accept secrets from the global secrets config used to generate manifests
export const graphqlMongoSettings: AppConfigs<"graphql-mongo", "mongodb", "applications"> = {
  kubeConfig: {
    requestMemory: "70Mi",
    requestCpu: "100m",
    limitMemory: "200Mi",
    limitCpu: "100m",
    replicaCount: 2,
    host: "0.0.0.0",
    image: `ghcr.io/oyelowo/graphql-mongo:${getEnvironmentVariables().IMAGE_TAG_GRAPHQL_MONGO}`,
  },

  envVars: {
    APP_ENVIRONMENT: getEnvironmentVariables().ENVIRONMENT,
    APP_HOST: "0.0.0.0",
    APP_PORT: "8000",

    MONGODB_NAME: "graphql-mongo-database",
    // TODO: remove these two. now coming handled in the deployment abstraction and uses referenced secret
    MONGODB_USERNAME: secretsFromLocalConfigs.MONGODB_USERNAME,
    MONGODB_PASSWORD: secretsFromLocalConfigs.MONGODB_PASSWORD,
    MONGODB_ROOT_USERNAME: secretsFromLocalConfigs.MONGODB_ROOT_USERNAME,
    MONGODB_ROOT_PASSWORD: secretsFromLocalConfigs.MONGODB_ROOT_PASSWORD,
    MONGODB_HOST: "graphql-mongo-database.applications",
    MONGODB_SERVICE_NAME: "graphql-mongo-database",
    MONGODB_STORAGE_CLASS: "linode-block-storage-retain",
    // TODO: ADD REDIS
    // hostAndPort":"graphql-mongo-0.mongo-graphql.development.svc.cluster.local:27017
    // MONGODB_HOST: "graphql-mongod-0.graphql-mongod-headless.development.svc.cluster.local",
    // const url = 'mongodb://username1:$[password]@mongo-graphql.development:27017/db1?authSource=$[authSource]';
    MONGODB_PORT: "27017",

    REDIS_USERNAME: secretsFromLocalConfigs.REDIS_USERNAME,
    REDIS_PASSWORD: secretsFromLocalConfigs.REDIS_PASSWORD,
    REDIS_HOST: "graphql-mongo-redis-master.applications",
    REDIS_SERVICE_NAME: "graphql-mongo-redis", // helm chart adds suffix to the name e.g (master) which the rust application must use as above
    REDIS_SERVICE_NAME_WITH_SUFFIX: "graphql-mongo-redis-master",
    REDIS_PORT: "6379",
  },
  metadata: {
    name: "graphql-mongo",
    namespace: namespaceNames.applications,
  },
};

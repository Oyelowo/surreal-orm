import { graphqlMongo } from "./index";
import { graphqlMongoSettings } from "./settings";
import { getEnvironmentVariables } from "./../shared/validations";
import { RedisHelmValuesBitnami } from "./../shared/types/helm-charts/redisHelmValuesBitnami";
import * as k8s from "@pulumi/kubernetes";

import { namespaceNames } from "../shared/namespaces";
import { DeepPartial } from "../shared/types/own-types";

const { envVars } = graphqlMongoSettings;

export const redisValues: DeepPartial<RedisHelmValuesBitnami> = {
  //   architecture: "replication",
  architecture: "standalone",

  // nameOverride: "redis-graphql",
  fullnameOverride: envVars.REDIS_SERVICE_NAME,
  replica: {
    replicaCount: 1,
  },
  global: {
    // namespaceOverride: devNamespaceName,
    storageClass:
      getEnvironmentVariables().ENVIRONMENT === "local"
        ? ""
        : graphqlMongoSettings.envVars.MONGODB_STORAGE_CLASS,
  },

  auth: {
    enabled: false, // TODO:: auth. Figure out how to connect with the FQDNurl with password in rust app ; graphql-mongo
    // password: envVars.REDIS_PASSWORD,
  },
  master: {
    service: {
      type: "ClusterIP",
      ports: {
        redis: 6379,

        //    Number(envVars.REDIS_PORT),
        // nameOverride: envVars.REDIS_SERVICE_NAME,
      },
      // portName: "mongo-graphql",
    },
  },
};

// `http://${name}.${namespace}:${port}`;
export const graphqlMongoRedis = new k8s.helm.v3.Chart(
  "redis",
  {
    chart: "redis",
    fetchOpts: {
      // TODO: Put in an object global shared variable
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "16.8.5",
    values: redisValues,
    namespace: namespaceNames.applications,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: graphqlMongo.getProvider() }
);

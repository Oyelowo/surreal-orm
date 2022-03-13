import * as k8s from "@pulumi/kubernetes";

import { provider } from "../shared/cluster";
import { devNamespaceName } from "../shared/namespaces";
import { RedisHelmValuesBitnami } from "../shared/redisHelmValuesBitnami";
import { DeepPartial } from "../shared/types";

// const { envVars } = graphqlMongoSettings;

export const redisValues: DeepPartial<RedisHelmValuesBitnami> = {
  //   architecture: "replication",
  architecture: "standalone",

  // nameOverride: "redis-graphql",
  fullnameOverride: "redis-database",
  replica: {
    replicaCount: 1,
  },
  // global: {
  //   namespaceOverride: devNamespaceName,
  // },

  auth: {
    enabled: true,
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
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "16.4.5",
    values: redisValues,
    namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider }
);

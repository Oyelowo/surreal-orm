import { graphqlPostgresEnvVars } from "./settings";

import { postgresdbHelmValuesBitnami } from "../shared/postgresdbHelmValuesBitnami";
import { devNamespaceName } from "../shared/namespaces";
import { DeepPartial, RecursivePartial } from "../shared/types";
import * as k8s from "@pulumi/kubernetes";
import { provider } from "../shared/cluster";

type Credentials = {
  usernames: string[];
  passwords: string[];
  databases: string[];
};
const credentials = [
  {
    username: graphqlPostgresEnvVars.POSTGRES_USERNAME,
    password: graphqlPostgresEnvVars.POSTGRES_PASSWORD,
    database: graphqlPostgresEnvVars.POSTGRES_NAME,
  },
  {
    username: "username1",
    password: "password1",
    database: "database1",
  },
  {
    username: "username2",
    password: "password2",
    database: "database2",
  },
  {
    username: "username3",
    password: "password3",
    database: "database1",
  },
  {
    username: "username4",
    password: "password4",
    database: "database2",
  },
];

const mappedCredentials = credentials.reduce<Credentials>(
  (acc, val) => {
    acc.usernames.push(val.username);
    acc.passwords.push(val.password);
    acc.databases.push(val.database);
    return acc;
  },
  {
    usernames: [],
    passwords: [],
    databases: [],
  }
);

const postgresValues: DeepPartial<postgresdbHelmValuesBitnami> = {
  // useStatefulSet: true,
  architecture: "standalone", //  "replication" | "standalone"
  // replicaCount: 3,
  // nameOverride: "postgres-database",
  fullnameOverride: graphqlPostgresEnvVars.POSTGRES_SERVICE_NAME,
  auth: {
    database: graphqlPostgresEnvVars.POSTGRES_DATABASE_NAME,
    postgresPassword: graphqlPostgresEnvVars.POSTGRES_PASSWORD,
    password: graphqlPostgresEnvVars.POSTGRES_PASSWORD,
    username: graphqlPostgresEnvVars.POSTGRES_USERNAME,
  },
  global: {
    // namespaceOverride: devNamespaceName,
    // imagePullSecrets: [],
    // storageClass: "",
    postgresql: {
      auth: {
        username: graphqlPostgresEnvVars.POSTGRES_USERNAME,
        password: graphqlPostgresEnvVars.POSTGRES_PASSWORD,
        database: graphqlPostgresEnvVars.POSTGRES_DATABASE_NAME,
        postgresPassword: graphqlPostgresEnvVars.POSTGRES_PASSWORD,
        // existingSecret: "",
      },
      service: {
        ports: {
          postgresql: graphqlPostgresEnvVars.POSTGRES_PORT,
        },
      },
    },
  },
//   primary: {
//     service: {
//       type: "ClusterIP",
//       ports: {
//         postgresql: Number(graphqlPostgresEnvironmentVariables.POSTGRES_PORT),
//       },
//     },
//   },

  //   service: {
  //     type: "ClusterIP",
  //     port: Number(graphqlPostgresEnvironmentVariables.POSTGRES_PORT),
  //     // portName: "mongo-graphql",
  //     // nameOverride: graphqlPostgresEnvironmentVariables.POSTGRES_SERVICE_NAME,
  //   },
};

export const graphqlPostgresPostgresdb = new k8s.helm.v3.Chart(
  "postgres-helm",
  {
    chart: "postgresql",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "11.0.6",
    values: postgresValues,
    namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider }
);

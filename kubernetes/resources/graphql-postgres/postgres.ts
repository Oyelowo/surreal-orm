import { graphqlPostgres } from "./index";
import { postgresdbHelmValuesBitnami } from "./../shared/types/helm-charts/postgresdbHelmValuesBitnami";
import * as k8s from "@pulumi/kubernetes";

import { namespaceNames } from "../shared/namespaces";
import { DeepPartial } from "../shared/types/own-types";
import { graphqlPostgresSettings } from "./settings";

const { envVars } = graphqlPostgresSettings;

type Credentials = {
  usernames: string[];
  passwords: string[];
  databases: string[];
};
const credentials = [
  {
    username: envVars.POSTGRES_USERNAME,
    password: envVars.POSTGRES_PASSWORD,
    database: envVars.POSTGRES_NAME,
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
  fullnameOverride: envVars.POSTGRES_SERVICE_NAME,
  auth: {
    database: envVars.POSTGRES_DATABASE_NAME,
    postgresPassword: envVars.POSTGRES_PASSWORD,
    password: envVars.POSTGRES_PASSWORD,
    username: envVars.POSTGRES_USERNAME,
  },
  global: {
    // namespaceOverride: devNamespaceName,
    // imagePullSecrets: [],
    // storageClass: "",
    postgresql: {
      auth: {
        username: envVars.POSTGRES_USERNAME,
        password: envVars.POSTGRES_PASSWORD,
        database: envVars.POSTGRES_DATABASE_NAME,
        postgresPassword: envVars.POSTGRES_PASSWORD,
        // existingSecret: "",
      },
      service: {
        ports: {
          postgresql: envVars.POSTGRES_PORT,
        },
      },
    },
    storageClass: envVars.APP_ENVIRONMENT === "local" ? "" : "linode-", // FIXME TODO: Specify the storage class here
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
    namespace: namespaceNames.applications,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: graphqlPostgres.getProvider() }
  // { provider }
);

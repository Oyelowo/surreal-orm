import { graphqlPostgresEnvVars } from "./settings";

import { postgresdbHaHelmValuesBitnami } from "../shared/postgresdbHAHelmValuesBitnami";
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

const postgresValues: DeepPartial<postgresdbHaHelmValuesBitnami> = {
  // useStatefulSet: true,
  // architecture: "replicaset",
  // replicaCount: 3,
  // nameOverride: "postgres-database",
  // nameOverride: graphqlPostgresEnvironmentVariables.POSTGRES_SERVICE_NAME,
  fullnameOverride: graphqlPostgresEnvVars.POSTGRES_SERVICE_NAME,
  postgresql: {
    // replicaCount: 3,
    // containerPort,
    username: graphqlPostgresEnvVars.POSTGRES_USERNAME,
    //pgHbaConfiguration: "",
    postgresPassword: graphqlPostgresEnvVars.POSTGRES_PASSWORD,
    database: graphqlPostgresEnvVars.POSTGRES_DATABASE_NAME,
    password: graphqlPostgresEnvVars.POSTGRES_PASSWORD,
   // repmgrPassword: graphqlPostgresEnvironmentVariables.POSTGRES_PASSWORD,
   // repmgrDatabase: graphqlPostgresEnvironmentVariables.POSTGRES_DATABASE_NAME,
    // existingSecret: "",
  },
  pgpool: {
    // existingSecret: "",
    // customUsers: "",
    // usernames: "",
    // passwords: "",
    // adminPassword: "",
    // adminUsername: "",
    replicaCount: 2,
  },
  global: {
    // namespaceOverride: devNamespaceName,
    // imagePullSecrets: [],
    //storageClass: "",
    pgpool: {
      // adminUsername: "",
      // adminPassword: "",
      // existingSecret: "",
    },
    postgresql: {
      // username: "",
      // password: "",
      // database: "",
      // repmgrUsername: "",
      // repmgrPassword: "",
      // repmgrDatabase: "",
      // existingSecret: "",
    },
    ldap: {},
  },
  service: {
    type: "ClusterIP",
    port: Number(graphqlPostgresEnvVars.POSTGRES_PORT),
    // portName: "mongo-graphql",
    // nameOverride: graphqlPostgresEnvironmentVariables.POSTGRES_SERVICE_NAME,
  },
};

// `http://${name}.${namespace}:${port}`;
export const graphqlPostgresPostgresdbHA = new k8s.helm.v3.Chart(
  "postgres-ha-helm",
  {
    chart: "postgresql-ha",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "8.4.0",
    values: postgresValues,
    namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider }
);

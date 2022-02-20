import { graphqlPostgresEnvironmentVariables } from "./settings";

import { postgresdbHaHelmValuesBitnami } from "../shared/postgresdbHaHelmValuesBitnami";
import { devNamespaceName } from "../shared/namespaces";
import { DeepPartial, RecursivePartial } from "../shared/types";
import * as k8s from "@pulumi/kubernetes";
import { provider } from "../shared/cluster";

/* MONGODB STATEFULSET */
type Credentials = {
  usernames: string[];
  passwords: string[];
  databases: string[];
};
const credentials = [
  {
    username: graphqlPostgresEnvironmentVariables.POSTGRES_USERNAME,
    password: graphqlPostgresEnvironmentVariables.POSTGRES_PASSWORD,
    database: graphqlPostgresEnvironmentVariables.POSTGRES_NAME,
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
  fullnameOverride: graphqlPostgresEnvironmentVariables.POSTGRES_SERVICE_NAME,
  postgresql: {
    replicaCount: 3,
    // containerPort,
    username: "",
    pgHbaConfiguration: "",
    postgresPassword: "",
    database: "",
    password: "",
    repmgrPassword: "",
    repmgrDatabase: "",
  },
  pgpool: {
    // existingSecret: "",
    customUsers: "",
    usernames: "",
    passwords: "",
    adminPassword: "",
    adminUsername: "",
  },
  global: {
    // namespaceOverride: devNamespaceName,
    imagePullSecrets: [],
    storageClass: "",
    pgpool: {
      adminUsername: "",
      adminPassword: "",
      existingSecret: "",
    },
    postgresql: {
      username: "",
      password: "",
      database: "",
      repmgrUsername: "",
      repmgrPassword: "",
      repmgrDatabase: "",
      // existingSecret: "",
    },
    ldap: {},
  },
  service: {
    type: "ClusterIP",
    port: Number(graphqlPostgresEnvironmentVariables.POSTGRES_PORT),
    // portName: "mongo-graphql",
    // nameOverride: graphqlPostgresEnvironmentVariables.POSTGRES_SERVICE_NAME,
  },
};

// `http://${name}.${namespace}:${port}`;
export const graphqlPostgresPostgresdb = new k8s.helm.v3.Chart(
  "postgres-helm",
  {
    chart: "postgresql-ha",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "8.4.0",
    values: {},
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider }
);

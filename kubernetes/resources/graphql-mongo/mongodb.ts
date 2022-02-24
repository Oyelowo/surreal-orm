import { graphqlMongoEnvVars } from "./settings";

import { MongodbHelmValuesBitnami } from "../shared/MongodbHelmValuesBitnami";
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
    username: graphqlMongoEnvVars.MONGODB_USERNAME,
    password: graphqlMongoEnvVars.MONGODB_PASSWORD,
    database: graphqlMongoEnvVars.MONGODB_NAME,
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

export const mongoValues: DeepPartial<MongodbHelmValuesBitnami> = {
  useStatefulSet: true,
  architecture: "replicaset",
  replicaCount: 3,
  // nameOverride: "mongodb-graphql",
  fullnameOverride: graphqlMongoEnvVars.MONGODB_SERVICE_NAME,
  // global: {
  //   namespaceOverride: devNamespaceName,
  // },
  persistence: {
    size: "0.1Gi",
  },

  auth: {
    enabled: true,
    rootUser: "root_user",
    rootPassword: "root_password",
    // array of
    ...mappedCredentials,
    // usernames: [graphqlMongoEnvironmentVariables.MONGODB_USERNAME],
    // passwords: [graphqlMongoEnvironmentVariables.MONGODB_PASSWORD],
    // databases: [graphqlMongoEnvironmentVariables.MONGODB_NAME],
    // users: [graphqlMongoEnvironmentVariables.MONGODB_USERNAME],
  },
  service: {
    type: "ClusterIP",
    port: Number(graphqlMongoEnvVars.MONGODB_PORT),
    // portName: "mongo-graphql",
    nameOverride: graphqlMongoEnvVars.MONGODB_SERVICE_NAME,
  },
};


// `http://${name}.${namespace}:${port}`;
export const graphqlMongoMongodb = new k8s.helm.v3.Chart(
  "graphql-mongodb-helm",
  {
    chart: "mongodb",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "11.0.3",
    values: mongoValues,
    namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider }
);
import { MongodbHelmValuesBitnami } from "../shared/MongodbHelmValuesBitnami";
import { grpcMongoEnvVars } from "./settings";

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
    username: grpcMongoEnvVars.MONGODB_USERNAME,
    password: grpcMongoEnvVars.MONGODB_PASSWORD,
    database: grpcMongoEnvVars.MONGODB_NAME,
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

const mongoValues: DeepPartial<MongodbHelmValuesBitnami> = {
  useStatefulSet: true,
  architecture: "replicaset",
  replicaCount: 3,
  // nameOverride: "mongodb-graphql",
  fullnameOverride: grpcMongoEnvVars.MONGODB_SERVICE_NAME,
  // global: {
  //   namespaceOverride: devNamespaceName,
  // },
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
    port: Number(grpcMongoEnvVars.MONGODB_PORT),
    // portName: "mongo-graphql",
    nameOverride: grpcMongoEnvVars.MONGODB_SERVICE_NAME,
  },
};

// `http://${name}.${namespace}:${port}`;
export const grpcMongoMongodb = new k8s.helm.v3.Chart(
  "grpc-mongodb-helm",
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

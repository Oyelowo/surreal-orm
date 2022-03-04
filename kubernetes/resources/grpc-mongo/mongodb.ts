import * as k8s from '@pulumi/kubernetes';

import { provider } from '../shared/cluster';
import { MongodbHelmValuesBitnami } from '../shared/MongodbHelmValuesBitnami';
import { devNamespaceName } from '../shared/namespaces';
import { DeepPartial, RecursivePartial } from '../shared/types';
import { grpcMongoSettings } from './settings';

/* MONGODB STATEFULSET */
type Credentials = {
  usernames: string[];
  passwords: string[];
  databases: string[];
};
const credentials = [
  {
    username: grpcMongoSettings.envVars.MONGODB_USERNAME,
    password: grpcMongoSettings.envVars.MONGODB_PASSWORD,
    database: grpcMongoSettings.envVars.MONGODB_NAME,
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
  fullnameOverride: grpcMongoSettings.envVars.MONGODB_SERVICE_NAME,
  // global: {
  //   namespaceOverride: devNamespaceName,
  // },
  auth: {
    enabled: true,
    rootUser: "root_user",
    rootPassword: "root_password",
    replicaSetKey: "replica_key",
    // array of
    ...mappedCredentials,
    // usernames: [graphqlMongoEnvironmentVariables.MONGODB_USERNAME],
    // passwords: [graphqlMongoEnvironmentVariables.MONGODB_PASSWORD],
    // databases: [graphqlMongoEnvironmentVariables.MONGODB_NAME],
    // users: [graphqlMongoEnvironmentVariables.MONGODB_USERNAME],
  },
  service: {
    type: "ClusterIP",
    port: Number(grpcMongoSettings.envVars.MONGODB_PORT),
    // portName: "mongo-graphql",
    nameOverride: grpcMongoSettings.envVars.MONGODB_SERVICE_NAME,
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

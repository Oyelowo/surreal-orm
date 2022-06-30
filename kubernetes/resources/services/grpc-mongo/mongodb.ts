import { IMongodbbitnami } from './../../types/helm-charts/mongodbBitnami';
import { grpcMongo } from './index';
import { namespaces } from '../../infrastructure/namespaces/util';
import * as k8s from '@pulumi/kubernetes';

import { grpcMongoSettings } from './settings';
import { DeepPartial } from '../../types/own-types';
import { getEnvironmentVariables } from '../../shared/validations';
import { helmChartsInfo } from '../../shared/helmChartInfo';

const environmentVariables = getEnvironmentVariables();

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
        username: 'username1',
        password: 'password1',
        database: 'database1',
    },
    {
        username: 'username2',
        password: 'password2',
        database: 'database2',
    },
    {
        username: 'username3',
        password: 'password3',
        database: 'database1',
    },
    {
        username: 'username4',
        password: 'password4',
        database: 'database2',
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

const mongoValues: DeepPartial<IMongodbbitnami> = {
    useStatefulSet: true,
    architecture: 'replicaset',
    replicaCount: 3,
    fullnameOverride: grpcMongoSettings.envVars.MONGODB_SERVICE_NAME,
    global: {
        storageClass:
            environmentVariables.ENVIRONMENT === 'local' ? '' : grpcMongoSettings.envVars.MONGODB_STORAGE_CLASS,
    },
    auth: {
        enabled: true,
        rootUser: 'root_user',
        rootPassword: 'root_password',
        replicaSetKey: 'Ld1My4Q1s4',
        ...(mappedCredentials as any),
    },
    service: {
        type: 'ClusterIP',
        port: Number(grpcMongoSettings.envVars.MONGODB_PORT),
        nameOverride: grpcMongoSettings.envVars.MONGODB_SERVICE_NAME,
    },
};

const {
    repo,
    charts: {
        mongodb: { chart, version },
    },
} = helmChartsInfo.bitnami;

// `http://${name}.${namespace}:${port}`;
export const grpcMongoMongodb = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: mongoValues,
        namespace: namespaces.applications,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: grpcMongo.getProvider() }
);

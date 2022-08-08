import { IMongodbbitnami } from '../../../../generatedHelmChartsTsTypes/mongodbBitnami.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../../infrastructure/namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { getEnvironmentVariables } from '../../shared/validations.js';
import { graphqlMongo } from './app.js';
import { graphqlMongoSettings } from './settings.js';

const { envVars } = graphqlMongoSettings;

/* MONGODB STATEFUL_SET */
type Credentials = {
    usernames: string[];
    passwords: string[];
    databases: string[];
};
const credentials = [
    {
        username: envVars.MONGODB_USERNAME,
        password: envVars.MONGODB_PASSWORD,
        database: envVars.MONGODB_NAME,
    },
    {
        username: 'username1',
        password: 'password1',
        database: 'db1',
    },
    {
        username: 'username2',
        password: 'password2',
        database: 'db2',
    },
];

const mappedCredentials: Credentials = {
    usernames: [],
    passwords: [],
    databases: [],
};

for (const val of credentials) {
    mappedCredentials.usernames.push(val.username);
    mappedCredentials.passwords.push(val.password);
    mappedCredentials.databases.push(val.database);
}

const mongoValues: DeepPartial<IMongodbbitnami> = {
    useStatefulSet: true,
    architecture: 'replicaset',
    replicaCount: 3,
    fullnameOverride: envVars.MONGODB_SERVICE_NAME,
    persistence: {
        /*
     Linode: This PVC represents a Block Storage Volume. Because Block Storage Volumes have a minimum size of 10 gigabytes, the storage has been set to 10Gi. If you choose a size smaller than 10 gigabytes, the PVC will default to 10 gigabytes.

Currently the only mode supported by the Linode Block Storage CSI driver is ReadWriteOnce, meaning that it can only be connected to one Kubernetes node at a time.
    */
        size: '10Gi', // Default is 8Gi.
        /*
    Note
In order to retain your Block Storage Volume and its data, even after the associated PVC is deleted, you must use the linode-block-storage-retain StorageClass. If, instead, you prefer to have your Block Storage Volume and its data deleted along with its PVC, use the linode-block-storage StorageClass. See the Delete a Persistent Volume Claim for steps on deleting a PVC.
    */
        storageClass: getEnvironmentVariables().ENVIRONMENT === 'local' ? '' : envVars.MONGODB_STORAGE_CLASS,
    },

    auth: {
        enabled: true,
        rootUser: envVars.MONGODB_ROOT_USERNAME,
        rootPassword: envVars.MONGODB_ROOT_PASSWORD,
        // replicaSetKey: 'Ld1My4Q1s4', // QUESTION: Should this be changed?
        // array of,
        ...mappedCredentials,
    },
    service: {
        type: 'ClusterIP',
        port: Number(envVars.MONGODB_PORT),
        nameOverride: envVars.MONGODB_SERVICE_NAME,
    },
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        mongodb: { chart, version },
    },
} = helmChartsInfo.bitnami;
export const graphqlMongoMongodb = new k8s.helm.v3.Chart(
    envVars.MONGODB_SERVICE_NAME,
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
    { provider: graphqlMongo.getProvider() }
);

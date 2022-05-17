import * as k8s from '@pulumi/kubernetes'
import { namespaceNames } from '../../namespaces/util'
import { MongodbHelmValuesBitnami } from '../../shared/types/helm-charts/MongodbHelmValuesBitnami'
import { DeepPartial } from '../../shared/types/own-types'
import { getEnvironmentVariables } from '../../shared/validations'
import { graphqlMongo } from './index'
import { graphqlMongoSettings } from './settings'


const { envVars } = graphqlMongoSettings

/* MONGODB STATEFUL_SET */
type Credentials = {
    usernames: string[]
    passwords: string[]
    databases: string[]
}
const credentials = [
    {
        username: '',
        password: '',
        database: '',
    },
    {
        username: '',
        password: '',
        database: '',
    },
    {
        username: '',
        password: '',
        database: '',
    },
    {
        username: '',
        password: '',
        database: '',
    },
    {
        username: '',
        password: '',
        database: '',
    },
]

const mappedCredentials = credentials.reduce<Credentials>(
    (acc, val) => {
        acc.usernames.push(val.username)
        acc.passwords.push(val.password)
        acc.databases.push(val.database)
        return acc
    },
    {
        usernames: [],
        passwords: [],
        databases: [],
    }
)

const mongoValues: DeepPartial<MongodbHelmValuesBitnami> = {
    useStatefulSet: true,
    architecture: 'replicaset',
    replicaCount: 3,
    // nameOverride: "mongodb-graphql",
    fullnameOverride: envVars.MONGODB_SERVICE_NAME,
    // global: {
    //   namespaceOverride: devNamespaceName,
    // },

    persistence: {
        /* 
     Linode: This PVC represents a Block Storage Volume. Because Block Storage Volumes have a minimum size of 10 gigabytes, the storage has been set to 10Gi. If you choose a size smaller than 10 gigabytes, the PVC will default to 10 gigabytes.

Currently the only mode supported by the Linode Block Storage CSI driver is ReadWriteOnce, meaning that it can only be connected to one Kubernetes node at a time.
    */
        size: '0.1Gi', // Default is 8Gi. // TODO: Confirm: This can be increased from initial but not decreased // TODO: Unset this or increase the capacity.
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
        replicaSetKey: 'Ld1My4Q1s4', // TODO
        // array of
        ...mappedCredentials,
        username: envVars.MONGODB_USERNAME,
        password: envVars.MONGODB_PASSWORD,
        // usernames: [graphqlMongoEnvironmentVariables.MONGODB_USERNAME],
        // passwords: [graphqlMongoEnvironmentVariables.MONGODB_PASSWORD],
        // databases: [graphqlMongoEnvironmentVariables.MONGODB_NAME],
        // users: [graphqlMongoEnvironmentVariables.MONGODB_USERNAME],
    },
    service: {
        type: 'ClusterIP',
        port: Number(envVars.MONGODB_PORT),
        // portName: "mongo-graphql",
        nameOverride: envVars.MONGODB_SERVICE_NAME,
    },
}

// `http://${name}.${namespace}:${port}`;
export const graphqlMongoMongodb = new k8s.helm.v3.Chart(
    envVars.MONGODB_SERVICE_NAME,
    {
        chart: 'mongodb',
        fetchOpts: {
            repo: 'https://charts.bitnami.com/bitnami',
        },
        version: '11.1.5',
        values: mongoValues,
        namespace: namespaceNames.applications,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: graphqlMongo.getProvider() }
)

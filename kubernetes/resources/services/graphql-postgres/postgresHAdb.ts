import { graphqlPostgres } from './index'
import { postgresdbHaHelmValuesBitnami } from '../../shared/types/helm-charts/postgresdbHaHelmValuesBitnami'
import * as k8s from '@pulumi/kubernetes'

import { graphqlPostgresSettings } from './settings'
import { DeepPartial } from '../../shared/types/own-types'
import { namespaceNames } from '../../namespaces/util'

const { envVars } = graphqlPostgresSettings
type Credentials = {
    usernames: string[]
    passwords: string[]
    databases: string[]
}
const credentials = [
    {
        username: envVars.POSTGRES_USERNAME,
        password: envVars.POSTGRES_PASSWORD,
        database: envVars.POSTGRES_NAME,
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

const postgresValues: DeepPartial<postgresdbHaHelmValuesBitnami> = {
    // useStatefulSet: true,
    // architecture: "replicaset",
    // replicaCount: 3,
    // nameOverride: "postgres-database",
    // nameOverride: graphqlPostgresEnvironmentVariables.POSTGRES_SERVICE_NAME,
    fullnameOverride: envVars.POSTGRES_SERVICE_NAME,
    postgresql: {
        // replicaCount: 3,
        // containerPort,
        username: envVars.POSTGRES_USERNAME,
        //pgHbaConfiguration: "",
        postgresPassword: envVars.POSTGRES_PASSWORD,
        database: envVars.POSTGRES_DATABASE_NAME,
        password: envVars.POSTGRES_PASSWORD,
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
        type: 'ClusterIP',
        port: Number(envVars.POSTGRES_PORT),
        // portName: "mongo-graphql",
        // nameOverride: graphqlPostgresEnvironmentVariables.POSTGRES_SERVICE_NAME,
    },
}

// `http://${name}.${namespace}:${port}`;
export const graphqlPostgresPostgresdbHA = new k8s.helm.v3.Chart(
    'postgres-ha',
    {
        chart: 'postgresql-ha',
        fetchOpts: {
            repo: 'https://charts.bitnami.com/bitnami',
        },
        version: '8.4.0',
        values: postgresValues,
        namespace: namespaceNames.applications,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: graphqlPostgres.getProvider() }
)

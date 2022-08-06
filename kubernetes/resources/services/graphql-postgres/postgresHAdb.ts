import { IPostgresqlhabitnami } from './../../types/helm-charts/postgresqlHaBitnami.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../../infrastructure/namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { graphqlPostgres } from './app.js';
import { graphqlPostgresSettings } from './settings.js';

const { envVars } = graphqlPostgresSettings;

const postgresValues: DeepPartial<IPostgresqlhabitnami> = {
    fullnameOverride: envVars.POSTGRES_SERVICE_NAME,
    postgresql: {
        username: envVars.POSTGRES_USERNAME,
        postgresPassword: envVars.POSTGRES_PASSWORD,
        database: envVars.POSTGRES_DATABASE_NAME,
        password: envVars.POSTGRES_PASSWORD,
    },
    pgpool: {
        replicaCount: 2,
    },
    global: {
        pgpool: {},
        postgresql: {},
        ldap: {},
        storageClass: envVars.POSTGRES_STORAGE_CLASS,
    },
    service: {
        type: 'ClusterIP',
        ports: {
            postgresql: Number(envVars.POSTGRES_PORT),
        },
    },
};

const {
    repo,
    charts: {
        postgresqlHA: { chart, version },
    },
} = helmChartsInfo.bitnami;

// `http://${name}.${namespace}:${port}`;
export const graphqlPostgresPostgresdbHA = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: postgresValues,
        namespace: namespaces.applications,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: graphqlPostgres.getProvider() }
);

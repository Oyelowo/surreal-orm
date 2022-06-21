import { IPostgresqlhabitnami } from './../../types/helm-charts/postgresqlHaBitnami';
import * as k8s from '@pulumi/kubernetes';
import { namespaceNames } from '../../namespaces/util';
import { helmChartsInfo } from '../../shared/helmChartInfo';
import { DeepPartial } from '../../types/own-types';
import { graphqlPostgres } from './index';
import { graphqlPostgresSettings } from './settings';

const { envVars } = graphqlPostgresSettings;

const postgresValues: DeepPartial<IPostgresqlhabitnami> = {
    fullnameOverride: envVars.POSTGRES_SERVICE_NAME,
    postgresql: {
        username: envVars.POSTGRES_USERNAME,
        postgresPassword: envVars.POSTGRES_PASSWORD,
        database: envVars.POSTGRES_DATABASE_NAME,
        password: envVars.POSTGRES_PASSWORD
    },
    pgpool: {
        replicaCount: 2,
    },
    global: {
        pgpool: {
        },
        postgresql: {

        },
        ldap: {},
        storageClass: envVars.POSTGRES_STORAGE_CLASS
    },
    service: {
        type: 'ClusterIP',
        ports: {
            postgresql: Number(envVars.POSTGRES_PORT),
        }
    },
};

const {
    repo,
    charts: { postgresqlHA: { chart, version } },
} = helmChartsInfo.bitnami;

// `http://${name}.${namespace}:${port}`;
export const graphqlPostgresPostgresdbHA = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo
        },
        version,
        values: postgresValues,
        namespace: namespaceNames.applications,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: graphqlPostgres.getProvider() }
);

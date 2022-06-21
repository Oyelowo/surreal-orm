import { IPostgresqlbitnami } from './../../types/helm-charts/postgresqlBitnami';
import * as k8s from '@pulumi/kubernetes';
import { namespaceNames } from '../../namespaces/util';
import { helmChartsInfo } from '../../shared/helmChartInfo';
import { DeepPartial } from '../../types/own-types';
import { graphqlPostgres } from './index';
import { graphqlPostgresSettings } from './settings';

const { envVars } = graphqlPostgresSettings;

const postgresValues: DeepPartial<IPostgresqlbitnami> = {
    architecture: 'standalone', //  "replication" | "standalone"
    fullnameOverride: envVars.POSTGRES_SERVICE_NAME,
    auth: {
        database: envVars.POSTGRES_DATABASE_NAME,
        postgresPassword: envVars.POSTGRES_PASSWORD,
        password: envVars.POSTGRES_PASSWORD,
        username: envVars.POSTGRES_USERNAME,
    },
    global: {
        postgresql: {
            auth: {
                username: envVars.POSTGRES_USERNAME,
                password: envVars.POSTGRES_PASSWORD,
                database: envVars.POSTGRES_DATABASE_NAME,
                postgresPassword: envVars.POSTGRES_PASSWORD,
            },
            service: {
                ports: {
                    postgresql: envVars.POSTGRES_PORT,
                },
            },
        },
        storageClass: envVars.APP_ENVIRONMENT === 'local' ? '' : envVars.POSTGRES_STORAGE_CLASS,
    },
};

const {
    repo,
    charts: {
        postgresql: { chart, version },
    },
} = helmChartsInfo.bitnami;

export const graphqlPostgresPostgresdb = new k8s.helm.v3.Chart(
    'postgresql',
    {
        chart,
        fetchOpts: {
            repo,
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

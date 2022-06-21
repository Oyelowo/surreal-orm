import { AppConfigs } from '../../types/own-types';
import { getEnvironmentVariables } from '../../shared/validations';

const environment = getEnvironmentVariables().ENVIRONMENT;

export const graphqlPostgresSettings: AppConfigs<'graphql-postgres', 'postgresdb', 'applications'> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 3,
        host: '0.0.0.0',
        readinessProbePort: 8000,
        image: `ghcr.io/oyelowo/graphql-postgres:${getEnvironmentVariables().IMAGE_TAG_GRAPHQL_POSTGRES}`,
    },

    envVars: {
        APP_ENVIRONMENT: environment,
        APP_HOST: '0.0.0.0',
        APP_PORT: '8000',
        POSTGRES_DATABASE_NAME: 'graphql-postgres-database',
        POSTGRES_NAME: 'graphql-postgres-database',
        POSTGRES_USERNAME: 'postgres',
        POSTGRES_PASSWORD: '1234', // TODO: Get from config above
        POSTGRES_HOST: 'graphql-postgres-database.applications', // the name of the postgres service being connected to. The name has suffices(primary|read etc) if using replcated architecture
        POSTGRES_PORT: '5432',
        POSTGRES_SERVICE_NAME: 'graphql-postgres-database',
        POSTGRES_STORAGE_CLASS: 'linode-block-storage-retain',
    },
    metadata: {
        name: 'graphql-postgres',
        namespace: 'applications',
    },
};

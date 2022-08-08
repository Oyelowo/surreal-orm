import { PlainSecretJsonConfig } from '../../../../scripts/utils/plainSecretJsonConfig.js';
import { AppConfigs } from '../../types/ownTypes.js';
import { getEnvironmentVariables } from '../../shared/validations.js';
import { getBaseUrl } from '../../infrastructure/ingress/hosts.js';

const environment = getEnvironmentVariables().ENVIRONMENT;
const secretsFromLocalConfigs = new PlainSecretJsonConfig('graphql-postgres', environment).getSecrets();

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
        APP_EXTERNAL_BASE_URL: getBaseUrl(environment),
        POSTGRES_DATABASE_NAME: 'graphql-postgres-database',
        POSTGRES_NAME: 'graphql-postgres-database',
        POSTGRES_USERNAME: secretsFromLocalConfigs.POSTGRES_USERNAME,
        POSTGRES_PASSWORD: secretsFromLocalConfigs.POSTGRES_PASSWORD,
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

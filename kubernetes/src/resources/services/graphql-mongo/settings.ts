import { AppConfigs } from '../../types/ownTypes.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';
import { getEnvVarsForKubeManifests } from '../../types/environmentVariables.js';

const env = getEnvVarsForKubeManifests();

export const graphqlMongoSettings: AppConfigs<'graphql-mongo', 'applications'> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 2,
        readinessProbePort: 8000,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/graphql-mongo:${env.SERVICES__GRAPHQL_MONGO__IMAGE_TAG}`,
    },

    envVars: {
        APP_ENVIRONMENT: env.ENVIRONMENT,
        APP_HOST: '0.0.0.0',
        APP_PORT: '8000',
        APP_EXTERNAL_BASE_URL: getIngressUrl({ environment: env.ENVIRONMENT }),
        OAUTH_GITHUB_CLIENT_ID: env.SERVICES__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_ID,
        OAUTH_GITHUB_CLIENT_SECRET: env.SERVICES__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_SECRET,
        OAUTH_GOOGLE_CLIENT_ID: env.SERVICES__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_ID,
        OAUTH_GOOGLE_CLIENT_SECRET: env.SERVICES__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_SECRET,

        MONGODB_NAME: 'graphql-mongo-database',
        MONGODB_USERNAME: env.SERVICES__GRAPHQL_MONGO__MONGODB_USERNAME,
        MONGODB_PASSWORD: env.SERVICES__GRAPHQL_MONGO__MONGODB_PASSWORD,
        MONGODB_ROOT_USERNAME: env.SERVICES__GRAPHQL_MONGO__MONGODB_ROOT_USERNAME,
        MONGODB_ROOT_PASSWORD: env.SERVICES__GRAPHQL_MONGO__MONGODB_ROOT_PASSWORD,
        MONGODB_HOST: 'graphql-mongo-database.applications',
        MONGODB_SERVICE_NAME: 'graphql-mongo-database',
        MONGODB_STORAGE_CLASS: 'linode-block-storage-retain',
        MONGODB_PORT: '27017',

        REDIS_USERNAME: env.SERVICES__GRAPHQL_MONGO__REDIS_USERNAME,
        REDIS_PASSWORD: env.SERVICES__GRAPHQL_MONGO__REDIS_PASSWORD,
        REDIS_HOST: 'graphql-mongo-redis-master.applications',
        REDIS_SERVICE_NAME: 'graphql-mongo-redis', // helm chart adds suffix to the name e.g (master) which the rust application must use as above
        REDIS_SERVICE_NAME_MASTER: 'graphql-mongo-redis-master',
        REDIS_PORT: '6379',
    },
    metadata: {
        name: 'graphql-mongo',
        namespace: 'applications',
    },
};

// graphqlMongoSettings.envVars

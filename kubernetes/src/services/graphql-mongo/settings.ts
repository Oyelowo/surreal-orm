import { AppConfigs, AppEnvVars, MongoDbEnvVars, OauthEnvVars, RedisDbEnvVars } from '../../types/ownTypes.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';
import { PlainSecretsManager } from '../../../scripts/utils/plainSecretsManager.js';
import { getEnvVarsForKubeManifests, imageTags } from '../../shared/environmentVariablesForManifests.js';

const env = getEnvVarsForKubeManifests();

const secrets = new PlainSecretsManager('services', 'graphql-mongo', 'local').getSecrets();

export type GraphqlMongoEnvVars = AppEnvVars &
    OauthEnvVars &
    MongoDbEnvVars<'applications'> &
    RedisDbEnvVars<'applications'> & { ADDITIONAL_IS_POSSIBLE: string };

export const graphqlMongoSettings: AppConfigs<'graphql-mongo', 'applications', GraphqlMongoEnvVars> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 2,
        readinessProbePort: 8000,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/graphql-mongo:${imageTags.SERVICES__GRAPHQL_MONGO__IMAGE_TAG}`,
    },
    envVars: {
        ADDITIONAL_IS_POSSIBLE: '',
        APP_HOST: '0.0.0.0',
        APP_PORT: '8000',
        APP_ENVIRONMENT: env.ENVIRONMENT,
        APP_EXTERNAL_BASE_URL: getIngressUrl({ environment: env.ENVIRONMENT }),
        OAUTH_GITHUB_CLIENT_ID: secrets.OAUTH_GITHUB_CLIENT_ID,
        OAUTH_GITHUB_CLIENT_SECRET: secrets.OAUTH_GITHUB_CLIENT_SECRET,
        OAUTH_GOOGLE_CLIENT_ID: secrets.OAUTH_GOOGLE_CLIENT_ID,
        OAUTH_GOOGLE_CLIENT_SECRET: secrets.OAUTH_GOOGLE_CLIENT_SECRET,

        MONGODB_NAME: 'mongodb',
        MONGODB_USERNAME: secrets.MONGODB_USERNAME,
        MONGODB_PASSWORD: secrets.MONGODB_PASSWORD,
        MONGODB_ROOT_USERNAME: secrets.MONGODB_ROOT_USERNAME,
        MONGODB_ROOT_PASSWORD: secrets.MONGODB_ROOT_PASSWORD,
        MONGODB_HOST: 'mongodb.applications',
        MONGODB_SERVICE_NAME: 'mongodb',
        MONGODB_STORAGE_CLASS: 'linode-block-storage-retain',
        MONGODB_PORT: '27017',

        REDIS_USERNAME: secrets.REDIS_USERNAME,
        REDIS_PASSWORD: secrets.REDIS_PASSWORD,
        REDIS_HOST: 'redis-master.applications',
        REDIS_SERVICE_NAME: 'redis', // helm chart adds suffix to the name e.g (master) which the rust application must use as above
        REDIS_SERVICE_NAME_MASTER: 'redis-master',
        REDIS_PORT: '6379',
    },
    metadata: {
        name: 'graphql-mongo',
        namespace: 'applications',
    },
};

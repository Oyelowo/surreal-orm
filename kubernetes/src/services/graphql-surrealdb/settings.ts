import { AppConfigs, AppEnvVars, OauthEnvVars } from '../../types/ownTypes.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';
import { PlainSecretsManager } from '../../../scripts/utils/plainSecretsManager.js';
import { getEnvVarsForKubeManifests, imageTags } from '../../shared/environmentVariablesForManifests.js';

const env = getEnvVarsForKubeManifests();

const secrets = new PlainSecretsManager('services', 'graphql-mongo', 'local').getSecrets();
type GraphqlSurrealdbEnvVars = AppEnvVars & OauthEnvVars;

export const graphqlSurrealdbSettings: AppConfigs<'graphql-surrealdb', 'applications', GraphqlSurrealdbEnvVars> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 2,
        readinessProbePort: 8000,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/graphql-surrealdb:${imageTags.SERVICES__GRAPHQL_SURREALDB__IMAGE_TAG}`,
    },

    envVars: {
        APP_ENVIRONMENT: env.ENVIRONMENT,
        APP_HOST: '0.0.0.0',
        APP_PORT: '8000',
        APP_EXTERNAL_BASE_URL: getIngressUrl({ environment: env.ENVIRONMENT }),
        OAUTH_GITHUB_CLIENT_ID: secrets.OAUTH_GITHUB_CLIENT_ID,
        OAUTH_GITHUB_CLIENT_SECRET: secrets.OAUTH_GITHUB_CLIENT_SECRET,
        OAUTH_GOOGLE_CLIENT_ID: secrets.OAUTH_GOOGLE_CLIENT_ID,
        OAUTH_GOOGLE_CLIENT_SECRET: secrets.OAUTH_GOOGLE_CLIENT_SECRET,
    },
    metadata: {
        name: 'graphql-surrealdb',
        namespace: 'applications',
    },
};

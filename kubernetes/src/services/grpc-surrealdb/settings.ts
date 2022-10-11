import { AppConfigs, AppEnvVars, OauthEnvVars, SurrealDbEnvVars } from '../../types/ownTypes.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';
import { PlainSecretsManager } from '../../../scripts/utils/plainSecretsManager.js';
import { getEnvVarsForKubeManifests, imageTags } from '../../shared/environmentVariablesForManifests.js';

const env = getEnvVarsForKubeManifests();

const secrets = new PlainSecretsManager('services', 'grpc-surrealdb', 'local').getSecrets();
export type GrpcSurrealDbEnvVars = AppEnvVars & OauthEnvVars & SurrealDbEnvVars<'applications'>;

export const grpcSurrealdbSettings: AppConfigs<'grpc-surrealdb', 'applications', GrpcSurrealDbEnvVars> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 2,
        readinessProbePort: 8000,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/grpc-surrealdb:${imageTags.SERVICES__GRPC_SURREALDB__IMAGE_TAG}`,
    },

    envVars: {
        APP_ENVIRONMENT: env.ENVIRONMENT,
        APP_HOST: '0.0.0.0',
        APP_PORT: '50051',
        APP_EXTERNAL_BASE_URL: getIngressUrl({ environment: env.ENVIRONMENT }),
        OAUTH_GITHUB_CLIENT_ID: secrets.OAUTH_GITHUB_CLIENT_ID,
        OAUTH_GITHUB_CLIENT_SECRET: secrets.OAUTH_GITHUB_CLIENT_SECRET,
        OAUTH_GOOGLE_CLIENT_ID: secrets.OAUTH_GOOGLE_CLIENT_ID,
        OAUTH_GOOGLE_CLIENT_SECRET: secrets.OAUTH_GOOGLE_CLIENT_SECRET,
        SURREALDB_HOST: 'surrealdb.applications',
        SURREALDB_NAME: 'surrealdb',
        SURREALDB_SERVICE_NAME: 'surrealdb',
        SURREALDB_PORT: '8000',
        SURREALDB_ROOT_USERNAME: secrets.SURREALDB_ROOT_USERNAME,
        SURREALDB_ROOT_PASSWORD: secrets.SURREALDB_ROOT_PASSWORD,
    },
    metadata: {
        name: 'grpc-surrealdb',
        namespace: 'applications',
    },
};

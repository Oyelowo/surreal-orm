import { getEnvironmentVariables } from '../../shared/validations.js';
import { AppConfigs } from '../../types/ownTypes.js';
import { getBaseUrl } from '../../infrastructure/ingress/hosts.js';
import { PlainSecretJsonConfig } from '../../../../scripts/utils/plainSecretJsonConfig.js';

const environmentVariables = getEnvironmentVariables();
const environment = environmentVariables.ENVIRONMENT;

// Maybe?: Rethink this abstraction for secret. Maybe can be gotten directly from the typescript secret file which is gitignored locally?
const secretsFromLocalConfigs = new PlainSecretJsonConfig('grpc-mongo', environment).getSecrets();

export const grpcMongoSettings: AppConfigs<'grpc-mongo', 'mongodb', 'applications'> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 3,
        readinessProbePort: 5000,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/grpc-mongo:${environmentVariables.IMAGE_TAG_GRPC_MONGO}`,
    },

    envVars: {
        APP_ENVIRONMENT: environment,
        APP_HOST: '0.0.0.0',
        APP_PORT: '50051',
        APP_EXTERNAL_BASE_URL: getBaseUrl(environment),
        MONGODB_NAME: 'grpc-mongo-database',
        MONGODB_USERNAME: secretsFromLocalConfigs.MONGODB_USERNAME,
        MONGODB_PASSWORD: secretsFromLocalConfigs.MONGODB_PASSWORD,
        MONGODB_ROOT_USERNAME: secretsFromLocalConfigs.MONGODB_ROOT_USERNAME,
        MONGODB_ROOT_PASSWORD: secretsFromLocalConfigs.MONGODB_ROOT_PASSWORD,
        MONGODB_HOST: 'grpc-mongo-database.applications',
        MONGODB_SERVICE_NAME: 'grpc-mongo-database',
        MONGODB_STORAGE_CLASS: 'linode-block-storage-retain',
        MONGODB_PORT: '27017',
    },
    metadata: {
        name: 'grpc-mongo',
        namespace: 'applications',
    },
};

import { AppConfigs, AppEnvVars, MongoDbEnvVars } from '../../types/ownTypes.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';
import { getEnvVarsForKubeManifests, imageTags } from '../../shared/environmentVariablesForManifests.js';
import { PlainSecretsManager } from '../../../scripts/utils/plainSecretsManager.js';

const env = getEnvVarsForKubeManifests();

const secrets = new PlainSecretsManager('services', 'grpc-mongo', env.ENVIRONMENT).getSecrets();

export type GrpcMongoEnvVars = AppEnvVars & MongoDbEnvVars<'applications'>;

export const grpcMongoSettings: AppConfigs<'grpc-mongo', 'applications', GrpcMongoEnvVars> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 3,
        readinessProbePort: 5000,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/grpc-mongo:${imageTags.SERVICES__GRPC_MONGO__IMAGE_TAG}`,
    },

    envVars: {
        APP_ENVIRONMENT: env.ENVIRONMENT,
        APP_HOST: '0.0.0.0',
        APP_PORT: '50051',
        APP_EXTERNAL_BASE_URL: getIngressUrl({ environment: env.ENVIRONMENT }),
        MONGODB_NAME: 'mongodb',
        MONGODB_USERNAME: secrets.MONGODB_USERNAME,
        MONGODB_PASSWORD: secrets.MONGODB_PASSWORD,
        MONGODB_ROOT_USERNAME: secrets.MONGODB_ROOT_USERNAME,
        MONGODB_ROOT_PASSWORD: secrets.MONGODB_ROOT_PASSWORD,
        MONGODB_HOST: 'mongodb.applications',
        MONGODB_SERVICE_NAME: 'mongodb',
        MONGODB_STORAGE_CLASS: 'linode-block-storage-retain',
        MONGODB_PORT: '27017',
    },
    metadata: {
        name: 'grpc-mongo',
        namespace: 'applications',
    },
};

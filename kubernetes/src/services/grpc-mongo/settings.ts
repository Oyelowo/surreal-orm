import { AppConfigs } from '../../types/ownTypes.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';
import { getEnvVarsForKubeManifests, imageTags } from '../../shared/environmentVariablesForManifests.js';
import { PlainKubeBuildSecretsManager } from '../../../scripts/utils/plainKubeBuildSecretsManager.js';

const env = getEnvVarsForKubeManifests();
const secrets = new PlainKubeBuildSecretsManager('services', 'grpc-mongo', env.ENVIRONMENT).getSecrets();

export const grpcMongoSettings: AppConfigs<'grpc-mongo', 'applications'> = {
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
        MONGODB_NAME: 'grpc-mongo-database',
        MONGODB_USERNAME: secrets.MONGODB_USERNAME,
        MONGODB_PASSWORD: secrets.MONGODB_PASSWORD,
        MONGODB_ROOT_USERNAME: secrets.MONGODB_ROOT_USERNAME,
        MONGODB_ROOT_PASSWORD: secrets.MONGODB_ROOT_PASSWORD,
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

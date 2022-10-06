import { AppConfigs, AppEnvVars } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests, imageTags } from '../../shared/environmentVariablesForManifests.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';

const env = getEnvVarsForKubeManifests();

const isLocal = env.ENVIRONMENT === 'local';

type ReactEnvVars = AppEnvVars;
export const reactWebSettings: AppConfigs<'react-web', 'applications', ReactEnvVars> = {
    kubeConfig: {
        requestMemory: isLocal ? '1.3Gi' : '300Mi',
        requestCpu: isLocal ? '500m' : '200m',
        limitMemory: isLocal ? '2Gi' : '300Mi',
        limitCpu: isLocal ? '700m' : '300m',
        replicaCount: 2,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/react-web:${imageTags.SERVICES__REACT_WEB__IMAGE_TAG}`,
    },

    envVars: {
        APP_ENVIRONMENT: env.ENVIRONMENT,
        APP_HOST: '0.0.0.0',
        APP_PORT: '3000',
        APP_EXTERNAL_BASE_URL: getIngressUrl({ environment: env.ENVIRONMENT }),
        // Not really used as all backend functionality has been moved to rust backend.
        // So, not using typescript for any backend work. Keeping for reference purpose
        // GRAPHQL_MONGO_URL: getFQDNFromSettings(graphqlMongoSettings), // Get Url mongoFQDN
    },
    metadata: {
        name: 'react-web',
        namespace: 'applications',
    },
};

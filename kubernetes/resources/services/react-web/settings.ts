import { AppConfigs } from '../../types/ownTypes.js';
import { getEnvironmentVariables } from '../../shared/validations.js';
import { getBaseUrl } from '../../infrastructure/ingress/hosts.js';

const environment = getEnvironmentVariables().ENVIRONMENT;

const isLocal = environment === "local";
export const reactWebSettings: AppConfigs<'react-web', 'doesNotHaveDb', 'applications'> = {
    kubeConfig: {
        requestMemory: isLocal ? '1.3Gi' : "300Mi",
        requestCpu: isLocal ? '500m' : '200m',
        limitMemory: isLocal ? '2Gi' : '300Mi',
        limitCpu: isLocal ? '700m' : "300m",
        replicaCount: 2,
        host: '0.0.0.0',
        image: `ghcr.io/oyelowo/react-web:${getEnvironmentVariables().IMAGE_TAG_REACT_WEB}`,
    },

    envVars: {
        APP_ENVIRONMENT: environment,
        APP_HOST: '0.0.0.0',
        APP_PORT: '3000',
        APP_EXTERNAL_BASE_URL: getBaseUrl(environment),
        // Not really used as all backend functionality has been moved to rust backend.
        // So, not using typescript for any backend work. Keeping for reference purpose
        // GRAPHQL_MONGO_URL: getFQDNFromSettings(graphqlMongoSettings), // Get Url mongoFQDN
    },
    metadata: {
        name: 'react-web',
        namespace: 'applications',
    },
};

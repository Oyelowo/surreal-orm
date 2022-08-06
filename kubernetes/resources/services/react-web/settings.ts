import { AppConfigs } from '../../types/own-types.js';
import { getEnvironmentVariables } from '../../shared/validations.js';
import { getBaseUrl } from '../../infrastructure/ingress/hosts.js';

const environment = getEnvironmentVariables().ENVIRONMENT;

export const reactWebSettings: AppConfigs<'react-web', 'doesNotHaveDb', 'applications'> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
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

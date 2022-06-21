import { getFQDNFromSettings } from '../../shared/helpers';
import { AppConfigs } from '../../types/own-types';
import { getEnvironmentVariables } from '../../shared/validations';
import { graphqlMongoSettings } from '../graphql-mongo/settings';

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
        APP_ENVIRONMENT: getEnvironmentVariables().ENVIRONMENT,
        APP_HOST: '0.0.0.0',
        APP_PORT: '3000',
        // Not really used as all backend functionality has been moved to rust backend. So, not using typescript for any backend work
        GRAPHQL_MONGO_URL: getFQDNFromSettings(graphqlMongoSettings), // Get Url mongoFQDN
    },
    metadata: {
        name: 'react-web',
        namespace: 'applications',
    },
};

import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const natsOperatorResourceName: ResourceName = 'nats-operator';
export const natsOperatorProvider = getResourceProvider({
    outputDirectory: `infrastructure/nats-operator`,
    environment: ENVIRONMENT,
});

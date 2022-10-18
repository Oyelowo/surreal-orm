import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const longhornOperatorResourceName: ResourceName = 'longhorn';
export const longhornOperatorProvider = getResourceProvider({
    outputDirectory: `infrastructure/longhorn`,
    environment: ENVIRONMENT,
});

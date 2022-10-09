import { getResourceProvider } from '../../shared/directoriesManager.js';
import { ResourceName } from '../../types/ownTypes.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const rookCephResourceName: ResourceName = 'rook-ceph';
export const rookCephProvider = getResourceProvider({
    outputDirectory: `infrastructure/rook-ceph`,
    environment: ENVIRONMENT,
});

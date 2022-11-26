import { getResourceProvider } from "../../shared/directoriesManager.js";
import { ResourceName } from "../../types/ownTypes.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const seaweedFsResourceName: ResourceName = "seaweedfs";
export const seaweedFsProvider = getResourceProvider({
	outputDirectory: "infrastructure/seaweedfs",
	environment: ENVIRONMENT,
});

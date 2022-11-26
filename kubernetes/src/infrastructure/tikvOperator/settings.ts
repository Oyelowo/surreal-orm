import { getResourceProvider } from "../../shared/directoriesManager.js";
import { ResourceName } from "../../types/ownTypes.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const tikvOperatorResourceName: ResourceName = "tikv-operator";
export const tikvOperatorProvider = getResourceProvider({
	outputDirectory: "infrastructure/tikv-operator",
	environment: ENVIRONMENT,
});

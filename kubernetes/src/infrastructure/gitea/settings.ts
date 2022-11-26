import { getResourceProvider } from "../../shared/directoriesManager.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const argoEventProvider = getResourceProvider({
	outputDirectory: "infrastructure/gitea",
	environment: ENVIRONMENT,
});

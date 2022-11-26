import { getResourceProvider } from "../../shared/directoriesManager.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const namespacesNamesProvider = getResourceProvider({
	outputDirectory: "infrastructure/namespaces",
	environment: ENVIRONMENT,
});

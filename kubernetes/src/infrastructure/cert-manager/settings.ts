import { getResourceProvider } from "../../shared/directoriesManager.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const certManagerProvider = getResourceProvider({
	outputDirectory: "infrastructure/cert-manager",
	environment: ENVIRONMENT,
});

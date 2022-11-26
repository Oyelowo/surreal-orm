import { getResourceProvider } from "../../shared/directoriesManager.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const argoWorkflowsProvider = getResourceProvider({
	outputDirectory: "infrastructure/argo-workflows",
	environment: ENVIRONMENT,
});

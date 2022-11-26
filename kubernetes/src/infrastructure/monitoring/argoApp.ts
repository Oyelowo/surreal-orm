import { createArgocdApplication } from "../argocd/createArgoApplication.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const monitoringApplication = createArgocdApplication({
	environment: ENVIRONMENT,
	sourceAppDirectory: "infrastructure/monitoring",
	outputDirectory: "infrastructure/argocd-applications-children-infrastructure",
	namespace: "monitoring",
});

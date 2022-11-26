import { createArgocdApplication } from "../argocd/createArgoApplication.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

// App that deploys metalbController resources themselves
/* metalbController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const metalbOperatorApplication = createArgocdApplication({
	sourceAppDirectory: "infrastructure/metalb",
	outputDirectory: "infrastructure/argocd-applications-children-infrastructure",
	environment: ENVIRONMENT,
	namespace: "metalb",
});

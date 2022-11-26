import { createArgocdApplication } from "../argocd/createArgoApplication.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const argoWorkflowsApplication = createArgocdApplication({
	environment: ENVIRONMENT,
	sourceAppDirectory: "infrastructure/argo-workflows",
	outputDirectory: "infrastructure/argocd-applications-children-infrastructure",
	namespace: "argo-workflows",
});

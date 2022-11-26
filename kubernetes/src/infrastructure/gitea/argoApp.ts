import { createArgocdApplication } from "../argocd/createArgoApplication.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const giteaApplication = createArgocdApplication({
	environment: ENVIRONMENT,
	sourceAppDirectory: "infrastructure/gitea",
	outputDirectory: "infrastructure/argocd-applications-children-infrastructure",
	namespace: "gitea",
});

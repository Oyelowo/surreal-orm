import { createArgocdApplication } from "../argocd/createArgoApplication.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const certManagerApplication = createArgocdApplication({
	environment: ENVIRONMENT,
	sourceAppDirectory: "infrastructure/cert-manager",
	outputDirectory: "infrastructure/argocd-applications-children-infrastructure",
	namespace: "cert-manager",
});

import { createArgocdApplication } from "../argocd/createArgoApplication.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

// App that deploys ciliumController resources themselves
/* cilium APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const ciliumApplication = createArgocdApplication({
	sourceAppDirectory: "infrastructure/cilium",
	outputDirectory: "infrastructure/argocd-applications-children-infrastructure",
	environment: ENVIRONMENT,
	namespace: "kube-system",
});

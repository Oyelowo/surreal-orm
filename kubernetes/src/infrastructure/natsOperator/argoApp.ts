import { createArgocdApplication } from "../argocd/createArgoApplication.js";
import { getEnvVarsForKubeManifests } from "../../shared/environmentVariablesForManifests.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

// App that deploys natsController resources themselves
/* natsOperator APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const natsOperatorApplication = createArgocdApplication({
	sourceAppDirectory: "infrastructure/nats-operator",
	outputDirectory: "infrastructure/argocd-applications-children-infrastructure",
	environment: ENVIRONMENT,
	namespace: "nats-operator",
});

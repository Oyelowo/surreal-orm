import { createArgocdApplication } from '../argocd/createArgoApplication.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

// App that deploys tikvController resources themselves
/* tikvOperator APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const tikvOperatorApplication = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/tikv',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'tikv-admin',
});

import { createArgocdApplication } from '../argocd/createArgoApplication.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

// App that deploys tikvController resources themselves
/* tikvOperator APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const fluvioSysApplication = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/fluvio-sys',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'fluvio-sys',
});

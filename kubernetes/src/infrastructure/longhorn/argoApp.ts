import { createArgocdApplication } from '../argocd/createArgoApplication.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

// App that deploys longhornController resources themselves
/* longhorn APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const longhornOperatorApplication = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/longhorn',
    outputDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    environment: ENVIRONMENT,
    namespace: 'longhorn-system',
});

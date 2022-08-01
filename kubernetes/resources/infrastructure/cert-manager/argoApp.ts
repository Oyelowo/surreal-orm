import { createArgocdApplication } from '../../shared/createArgoApplication.js';

export const certManagerApplication = createArgocdApplication({
    sourceApplication: 'cert-manager',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: 'cert-manager',
});

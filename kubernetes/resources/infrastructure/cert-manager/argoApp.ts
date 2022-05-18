import { createArgocdApplication } from '../../shared/createArgoApplication';

export const certManagerApplication = createArgocdApplication({
    // resourceName: 'argocd-applications-children-infrastructure',
    sourceApplication: 'cert-manager',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: 'cert-manager',
});

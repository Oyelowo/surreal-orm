import { createArgocdApplication } from '../../shared/createArgoApplication';

export const certManagerApplication = createArgocdApplication({
    resourceName: 'argocd-applications-children-infrastructure',
    sourceResourceName: 'cert-manager',
    namespace: 'cert-manager',
});

import { createArgocdApplication } from '../../shared/createArgoApplication';

export const certManagerApplication = createArgocdApplication({
    resourceName: 'cert-manager',
    sourceResourceName: 'argocd-applications-children-infrastructure',
    namespace: 'cert-manager',
});

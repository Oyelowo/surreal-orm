import { createArgocdApplication } from '../../shared/createArgoApplication';

export const ingressControllerApplication = createArgocdApplication({
    resourceName: 'nginx-ingress',
    sourceResourceName: 'argocd-applications-children-infrastructure',
    namespace: 'default',
});

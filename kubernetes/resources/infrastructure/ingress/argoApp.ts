import { createArgocdApplication } from '../../shared/createArgoApplication';

export const ingressControllerApplication = createArgocdApplication({
    resourceName: 'argocd-applications-children-infrastructure',
    sourceResourceName: 'nginx-ingress',
    namespace: 'default',
});

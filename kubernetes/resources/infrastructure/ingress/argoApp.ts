import { createArgocdApplication } from '../../shared/createArgoApplication';

export const ingressControllerApplication = createArgocdApplication({
    sourceResourceName: 'argocd-applications-children-infrastructure',
    resourceName: 'nginx-ingress',
    namespace: 'default',
});

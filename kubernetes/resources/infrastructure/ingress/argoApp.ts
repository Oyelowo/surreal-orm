import { createArgocdApplication } from '../../shared/createArgoApplication';

export const ingressControllerApplication = createArgocdApplication({
    sourceApplication: 'nginx-ingress',
    outputSubDirName: 'argocd-applications-children-infrastructure',
    namespace: 'default',
});

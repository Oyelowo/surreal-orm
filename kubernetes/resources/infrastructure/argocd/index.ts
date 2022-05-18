import { createArgocdApplication } from '../../shared/createArgoApplication';

// // I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from './argocdBitnami';
// // export * from "./argocdOfficial";

export const argoInfrastructureParentApplications = createArgocdApplication({
    resourceName: 'argocd-applications-parents',
    sourceResourceName: 'argocd-applications-children-infrastructure',
    namespace: 'argocd',
});

export const argoServicesParentApplications = createArgocdApplication({
    resourceName: 'argocd-applications-parents',
    sourceResourceName: 'argocd-applications-children-services',
    namespace: 'argocd',
});

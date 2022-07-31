import { createArgocdApplication } from '../../shared/createArgoApplication';

// I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from './argocdBitnami';
// export * from "./argocdOfficial";

export const argoInfrastructureParentApplications = createArgocdApplication({
    sourceApplication: 'argocd-applications-children-infrastructure',
    outputSubDirName: 'argocd-applications-parents',
    namespace: 'argocd',
});

export const argoServicesParentApplications = createArgocdApplication({
    sourceApplication: 'argocd-applications-children-services',
    outputSubDirName: 'argocd-applications-parents',
    namespace: 'argocd',
});

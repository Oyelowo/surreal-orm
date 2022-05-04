import { createArgocdParentsApplication } from '../../shared/createArgoApplication';

// // I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from "./argocdBitnami";
// // export * from "./argocdOfficial";

export const argoInfrastructureParentApplications = createArgocdParentsApplication({
    name: "infrastructure-aparent-pplication",
    namespace: "argocd",
    resourceType: "infrastructure",

})

export const argoServicesParentApplications = createArgocdParentsApplication({
    name: "infrastructure-services-application",
    namespace: "argocd",
    resourceType: "services",

})

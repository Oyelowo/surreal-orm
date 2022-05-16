import { createArgocdChildrenApplication } from '../../shared/createArgoApplication';
import { getEnvironmentVariables } from '../../shared/validations';
// import { createContainerRegistrySecret } from './docker';

// // I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from "./argocdBitnami";
// // export * from "./argocdOfficial";

export const argoInfrastructureParentApplications = createArgocdChildrenApplication({
    argoResourceType: "argocd-applications-parents",
    resourceName: "argocd-applications-children-infrastructure",
    // name: "infrastructure-parent-application",
    namespace: "argocd",
    // resourceType: "infrastructure",

})

export const argoServicesParentApplications = createArgocdChildrenApplication({
    // resourceDir
    argoResourceType: "argocd-applications-parents",
    // SourceResourceDir
    resourceName: "argocd-applications-children-services",
    // name: "infrastructure-services-application",
    namespace: "argocd",
    // resourceType: "services",

})

// createContainerRegistrySecret(getEnvironmentVariables().ENVIRONMENT)
// export * from "./docker";
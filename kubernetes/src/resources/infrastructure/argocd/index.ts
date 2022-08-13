import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

// I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from './argocdBitnami.js';
// export * from "./argocdOfficial.js";

const { ENVIRONMENT } = getEnvironmentVariables();
export const argoInfrastructureParentApplications = createArgocdApplication({
    resourceType: 'infrastructure',
    environment: ENVIRONMENT,
    sourceApplication: 'argocd-applications-children-infrastructure',
    outputSubDirName: 'argocd-applications-parents',
    namespace: 'argocd',
});

export const argoServicesParentApplications = createArgocdApplication({
    resourceType: 'infrastructure',
    environment: ENVIRONMENT,
    sourceApplication: 'argocd-applications-children-services',
    outputSubDirName: 'argocd-applications-parents',
    namespace: 'argocd',
});

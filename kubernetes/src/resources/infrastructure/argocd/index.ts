import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvironmentVariables } from '../../shared/validations.js';

// I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from './argocdBitnami.js';
// export * from "./argocdOfficial.js";

const { ENVIRONMENT } = getEnvironmentVariables();
export const argoInfrastructureParentApplications = createArgocdApplication({
    sourceApplicationName: 'argocd-applications-children-infrastructure',
    sourceApplicationPath: 'infrastructure/argocd-applications-children-infrastructure',
    outputPath: 'infrastructure/argocd-applications-parents',
    environment: ENVIRONMENT,
    namespace: 'argocd',
});

export const argoServicesParentApplications = createArgocdApplication({
    sourceApplicationName: 'argocd-applications-children-services',
    sourceApplicationPath: 'infrastructure/argocd-applications-children-services',
    outputPath: 'infrastructure/argocd-applications-parents',
    environment: ENVIRONMENT,
    namespace: 'argocd',
});

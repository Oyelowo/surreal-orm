import { createArgocdApplication } from '../../shared/createArgoApplication.js';
import { getEnvVarsForKubeManifests } from '../../types/environmentVariables.js';

// I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from './argocdBitnami.js';
// export * from "./argocdOfficial.js";

const { ENVIRONMENT } = getEnvVarsForKubeManifests();

export const argoInfrastructureParentApplications = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/argocd-applications-children-infrastructure',
    outputDirectory: 'infrastructure/argocd-applications-parents',
    environment: ENVIRONMENT,
    namespace: 'argocd',
});

export const argoServicesParentApplications = createArgocdApplication({
    sourceAppDirectory: 'infrastructure/argocd-applications-children-services',
    outputDirectory: 'infrastructure/argocd-applications-parents',
    environment: ENVIRONMENT,
    namespace: 'argocd',
});

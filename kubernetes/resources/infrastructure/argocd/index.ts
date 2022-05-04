import { getEnvironmentVariables } from "./../../shared/validations";
import {
    ArgoParentApplications,
    ArgoResourceName,
    getRelativePathToArgocdChildrenResource,
    ResourceName,
    ResourceType,
} from "./../../shared/manifestsDirectory";
// // Use either, Both work, but I'm using the offical one for now as it is stable and has support for notification.
import * as k8s from "@pulumi/kubernetes";

import { createArgocdApplication } from "../../shared/createArgoApplication";

// // I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from "./argocdBitnami";
// // export * from "./argocdOfficial";

// import { namespaceNames } from "../../namespaces";
// import { createArgocdApplication } from "../../shared/createArgoApplication";
// import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
// import { getEnvironmentVariables } from "../../shared/validations";
// import * as k8s from "@pulumi/kubernetes";

// const { ENVIRONMENT } = getEnvironmentVariables();
// export const argocdApplicationsDir = getArgocdInfraApplicationsDir(ENVIRONMENT);

// export const argocdController = createArgocdApplication({
//     metadata: {
//         name: argocdControllerName,
//         namespace: namespaceNames.argocd,
//         resourceType: "infrastructure"
//         // argoApplicationName: "argocd"
//     },
//     pathToAppManifests: getRepoPathFromAbsolutePath(argocdApplicationsDir),
// });

// getPathToResoucrceTypeDir()
const { ENVIRONMENT } = getEnvironmentVariables();
function getResourceTypeProvider(resourceType: ResourceType) {
    new k8s.Provider(`${resourceType}`, {
        renderYamlToDirectory: getRelativePathToArgocdChildrenResource(resourceType, ENVIRONMENT),
    });
}
const resourceTypes: Exclude<ResourceType, ArgoParentApplications>[] = [
    "infrastructure",
    "namespaces",
    "services",
];

export const resourceTypesApp = resourceTypes.map((resourceType) => {
    const resourceName: ArgoResourceName = `${resourceType}-argocd-parent-applications`
    const argoAppsParentsApplications = createArgocdApplication({
        namespace: "argocd",
        resourceName,
        sourcePath: getRelativePathToArgocdChildrenResource(resourceType, ENVIRONMENT)
    });
    return argoAppsParentsApplications
});


// export const argoAppsParentsApplications3 = createArgocdApplication({
//     metadata: {
//         name: "argo-applications-parents-services",
//         namespace: namespaceNames.argocd,
//         resourceType: "argo_applications_parents"
//         // argoApplicationName: "cert-manager"
//     },
//     pathToAppManifests: getRepoPathFromAbsolutePath(getArgocdServicesApplicationsDir(ENVIRONMENT)),
//     // pathToAppManifests: getRepoPathFromAbsolutePath(getArgoAppsParentsDir(ENVIRONMENT)),
// });
// export const argoAppsParentsApplications4 = createArgocdApplication({
//     metadata: {
//         name: "argo-applications-parents-namespaces",
//         namespace: namespaceNames.argocd,
//         resourceType: "argo_applications_parents"
//         // argoApplicationName: "cert-manager"
//     },
//     pathToAppManifests: getRepoPathFromAbsolutePath(getNamespacesNamesArgoAppDir(ENVIRONMENT)),
//     // pathToAppManifests: getRepoPathFromAbsolutePath(getArgoAppsParentsDir(ENVIRONMENT)),
// });

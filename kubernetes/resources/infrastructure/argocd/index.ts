import { getEnvironmentVariables } from "./../../shared/validations";
import {
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

// Argocd controller itself
const argoAppsParentsApplications = createArgocdApplication({
    resourceName: "argocd",
    namespace: "argocd",
    // sourcePath: getRelativePathToArgocdChildrenResource(resourceType, ENVIRONMENT)
});

export const argoAppsOfApp: ResourceName[] = [
    // "argocd-children-applications",
    // "argocd-parent-applications"
];


export const resourceTypesApp = argoAppsOfApp.map((resourceName) => {
    const argoAppsParentsApplications = createArgocdApplication({
        resourceName,
        namespace: "argocd",
        isParentApp: true
        // sourcePath: getRelativePathToArgocdChildrenResource(resourceType, ENVIRONMENT)
    });
    return argoAppsParentsApplications
});

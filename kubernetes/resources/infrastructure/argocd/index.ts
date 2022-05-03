import { argoAppsParentsProvider } from './../../shared/createArgoApplication';
import { argocdControllerName, getArgocdInfraApplicationsDir, getArgoAppsParentsDir } from './../../shared/manifestsDirectory';
// Use either, Both work, but I'm using the offical one for now as it is stable and has support for notification. 

// I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from "./argocdBitnami";
// export * from "./argocdOfficial";


import { namespaceNames } from "../../namespaces";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { argocdApplicationsName, getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";
import * as k8s from "@pulumi/kubernetes";



const { ENVIRONMENT } = getEnvironmentVariables();
export const argocdApplicationsDir = getArgocdInfraApplicationsDir(ENVIRONMENT);


// export const argocdApplicationsProvider = new k8s.Provider(argocdApplicationsDir, {
//     renderYamlToDirectory: argocdApplicationsDir,
// });


// export const argocdApplications = createArgocdApplication({
//     metadata: {
//         name: argocdApplicationsName,
//         namespace: namespaceNames.argocd,
//         // argoApplicationName: "argocd-applications"
//     },
//     pathToAppManifests: getRepoPathFromAbsolutePath(argocdApplicationsDir),
// });

export const argocdController = createArgocdApplication({
    metadata: {
        name: argocdControllerName,
        namespace: namespaceNames.argocd,
        resourceType: "infrastructure"
        // argoApplicationName: "argocd"
    },
    pathToAppManifests: getRepoPathFromAbsolutePath(argocdApplicationsDir),
});


export const argoAppsParentsApplications = createArgocdApplication({
    metadata: {
        name: "argo-applications-parents",
        namespace: namespaceNames.argocd,
        resourceType: "argo_applications_parents"
        // argoApplicationName: "cert-manager"
    },
    pathToAppManifests: getRepoPathFromAbsolutePath(getArgoAppsParentsDir(ENVIRONMENT)),
});

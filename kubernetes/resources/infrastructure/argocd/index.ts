// Use either, Both work, but I'm using the offical one for now as it is stable and has support for notification. 

// I am keeping bitnami version in the meantime for reference purpose. 26th April, 2022.
export * from "./argocdBitnami";
// export * from "./argocdOfficial";


import { namespaceNames } from "../../namespaces";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { argocdApplicationsName, getArgocdApplicationsDir, getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";
import * as k8s from "@pulumi/kubernetes";



const { ENVIRONMENT } = getEnvironmentVariables();
export const argocdApplicationsDir = getArgocdApplicationsDir(ENVIRONMENT);


export const argocdApplicationsProvider = new k8s.Provider(argocdApplicationsDir, {
    renderYamlToDirectory: argocdApplicationsDir,
});


export const argocdApplications = createArgocdApplication({
    metadata: {
        name: argocdApplicationsName,
        namespace: namespaceNames.argocd,
    },
    pathToAppManifests: getRepoPathFromAbsolutePath(argocdApplicationsDir),
});

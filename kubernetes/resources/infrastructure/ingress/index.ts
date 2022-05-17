// import { createArgocdApplication, namespaceNames } from "../shared";
// import { getPathToNonApplicationDir } from "../shared/manifestsDirectory";

export * from './ingressController'
export * from './argoApp'
// export * from "./certificate";

// const ingressControllerControllerDir = getPathToNonApplicationDir("ingressController-controller");

// type Metadata = {
//   name: string;
//   namespace: string;
// };

// const metadataIngressController: Metadata = {
//   name: "ingress-controller-application",
//   namespace: namespaceNames.default,
// };

// // App that deploys ingressController resources themselves
// /* ingressController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
// export const ingressControllerApplication = createArgocdApplication({
//   metadata: { ...metadataIngressController },
//   pathToAppManifests: ingressControllerControllerDir,
// });

// const metadataIngressRules: Metadata = {
//   name: "ingress-controller-application",
//   namespace: namespaceNames.applications,
// };

// export const ingressRulesApplication = createArgocdApplication({
//   metadata: { ...metadataIngressRules },
//   pathToAppManifests: ingressControllerControllerDir,
// });

// export * from "./sealedSecrets";

// import { createArgocdApplication, namespaceNames } from "../shared";
// import {
//   getPathToNonApplicationDir,
//   getSealedSecretsDirForEnv,
// } from "../shared/manifestsDirectory";

// export const sealedSecretsControllerDir = getPathToNonApplicationDir("sealed-secrets-controller");

// type Metadata = {
//   name: string;
//   namespace: string;
// };

// const metadataSealedSecretsController: Metadata = {
//   name: "ingress-controller-application",
//   namespace: namespaceNames.default,
// };

// // App that deploys sealedSecretsController resources themselves
// /* sealedSecretsController APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
// export const sealedSecretsControllerApplication = createArgocdApplication({
//   metadata: { ...metadataSealedSecretsController },
//   pathToAppManifests: sealedSecretsControllerDir,
// });


// const metadataIngressRules: Metadata = {
//   name: "sealed-secrets-application",
//   namespace: namespaceNames.applications,
// };

// export const ingressRulesApplication = createArgocdApplication({
//   metadata: { ...metadataIngressRules },
//   //   Sealed secrets are put here
//   pathToAppManifests: getSealedSecretsDirForEnv(),
// });

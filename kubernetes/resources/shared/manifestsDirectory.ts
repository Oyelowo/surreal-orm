import { AppName, Environment } from "./types/own-types";
import * as k8s from "@pulumi/kubernetes";
import * as pulumi from "@pulumi/pulumi";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import * as kx from "@pulumi/kubernetesx";
import { getEnvironmentVariables } from "./validations";
import path from "path";
// import { devNamespace } from "./namespaces";

// import * as eks from "@pulumi/eks";

// export const namespace = new k8s.Name;

// const nameSpaceName = "development";
const { ENVIRONMENT, TEMPORARY_DIR } = getEnvironmentVariables();

//  I am first putting all resources in a single cluster and allocating resources and envronment based on namespace rather than cluster.
// i.e  type Namespace = "development" | "staging" | "production". And only a single cluster.

// If need be, in the future, we can have three providers(clusters):
// type Cluster = "development" | "staging" | "production".
// while namespace can then be used for categorising resources based on logical grouping or team allocation. e.g
// type Namespace = "team-a" | "workers" | "web" | "jobs"

// TODO: probably use path and __dirname modules?
export const getManifestsOutputDirectory = (environment: Environment) =>
  path.join("manifests", "generated", environment);

const MANIFESTS_BASE_DIR_FOR_ENV = getManifestsOutputDirectory(ENVIRONMENT);
export const APPLICATION_DIR = path.join(MANIFESTS_BASE_DIR_FOR_ENV, "applications");

export const getPathToApplicationDir = (appName: AppName) => path.join(APPLICATION_DIR, appName);
export const getPathToNonApplicationDir = (toolName: string) =>
  path.join(MANIFESTS_BASE_DIR_FOR_ENV, toolName);

export const nameSpacesProvider = new k8s.Provider("render-namespaces", {
  renderYamlToDirectory: `${MANIFESTS_BASE_DIR_FOR_ENV}/namespaces`,
});

// Stores resources useful for starting a fresh cluster such as the
// sealed secrets controller and ingress controller which are
// fundamental for the applications that would run in the cluster
export const sealedSecretsControllerProvider = new k8s.Provider("sealed-secrets-controller", {
  renderYamlToDirectory: `${MANIFESTS_BASE_DIR_FOR_ENV}/sealed-secrets-controller`,
});
export const ingressControllerProvider = new k8s.Provider("ingress-controller", {
  renderYamlToDirectory: `${MANIFESTS_BASE_DIR_FOR_ENV}/ingress-controller`,
});



// export const devNamespace = new k8s.core.v1.Namespace(
//   "local",
//   {
//     metadata: { name: nameSpaceName, namespace: nameSpaceName },
//   },
//   { provider }
// );

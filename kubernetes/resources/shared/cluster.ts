import * as k8s from "@pulumi/kubernetes";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import * as kx from "@pulumi/kubernetesx";
import { environmentVariables } from "./validations";

// import { devNamespace } from "./namespaces";

// import * as eks from "@pulumi/eks";

// export const namespace = new k8s.Name;

// const nameSpaceName = "development";
const { ENVIRONMENT, TEMPORARY_DIR } = environmentVariables;

//  I am first putting all resources in a single cluster and allocating resources and envronment based on namespace rather than cluster.
// i.e  type Namespace = "development" | "staging" | "production". And only a single cluster.

// If need be, in the future, we can have three providers(clusters):
// type Cluster = "development" | "staging" | "production".
// while namespace can then be used for categorising resources based on logical grouping or team allocation. e.g
// type Namespace = "team-a" | "workers" | "web" | "jobs"

const rootDirectory = `manifests/${ENVIRONMENT}/${
  TEMPORARY_DIR ?? "generated"
}`;
export const providerApplication = new k8s.Provider("render-yaml", {
  renderYamlToDirectory: `${rootDirectory}/applications`,
  // renderYamlToDirectory: `${rootDirectory}/${nameSpaceName}`,
  // namespace: "nana",
});

export const providerSecrets = new k8s.Provider("render-yaml-secrets", {
  renderYamlToDirectory: `${rootDirectory}/secrets`,
  // renderYamlToDirectory: `${rootDirectory}/${nameSpaceName}`,
  // namespace: "nana",
});

export const providerNameSpacesProvider = new k8s.Provider("render-yaml2", {
  renderYamlToDirectory: `${rootDirectory}/namespaces`,
  // namespace: "nana",
});

export const clusterSetupProvider = new k8s.Provider("cluster-setup", {
  renderYamlToDirectory: `${rootDirectory}/cluster-setup`,
  // namespace: "nana",
});

export const argoCDProvider = new k8s.Provider("render-argocd", {
  renderYamlToDirectory: `${rootDirectory}/argocd`,
  // namespace: "nana",
});

// export const devNamespace = new k8s.core.v1.Namespace(
//   "local",
//   {
//     metadata: { name: nameSpaceName, namespace: nameSpaceName },
//   },
//   { provider }
// );

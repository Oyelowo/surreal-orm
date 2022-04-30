import * as k8s from "@pulumi/kubernetes";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import { getGeneratedEnvManifestsDir } from "./manifestsDirectory";
import { getEnvironmentVariables } from "./validations";
import path from "path";

const { ENVIRONMENT } = getEnvironmentVariables();

const MANIFESTS_BASE_DIR_FOR_ENV = getGeneratedEnvManifestsDir(ENVIRONMENT);

export const nameSpacesProvider = new k8s.Provider("render-namespaces", {
  renderYamlToDirectory: path.join(MANIFESTS_BASE_DIR_FOR_ENV, "namespaces"),
});

// export const devNamespaceName = devNamespace.metadata.name as unknown as string;
export const namespaceNames = {
  applications: "applications",
  argocd: "argocd",
  certManager: "cert-manager",
  linkerd: "linkerd",
  linkerdViz: "linkerd-viz",
  default: "default",
  // Default namespace that comes with the deployment
  kubeSystem: "kube-system",
  // infrastructure: "infrastructure",
} as const;

type Keys = keyof typeof namespaceNames;
export type NamespaceName = typeof namespaceNames[Keys];

export const resourceNamespaces = Object.entries(namespaceNames).map(([_k, namespace]) => {
  const resourceNamespace = new Namespace(
    namespace,
    {
      metadata: {
        name: namespace,
        namespace,
      },
    },
    { provider: nameSpacesProvider }
  );
  return resourceNamespace
})
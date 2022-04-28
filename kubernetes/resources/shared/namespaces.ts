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
  default: "default",
  // Default namespace that comes with the deployment
  kubeSystem: "kube-system",
  // infrastructure: "infrastructure",
} as const;

type Keys = keyof typeof namespaceNames;
export type NamespaceName = typeof namespaceNames[Keys];

export const applicationsNamespace = new Namespace(
  namespaceNames.applications,
  {
    metadata: {
      name: `${namespaceNames.applications}`,
      namespace: namespaceNames.applications,
    },
  },
  { provider: nameSpacesProvider }
);

export const argocdNamespace = new Namespace(
  namespaceNames.argocd,
  {
    metadata: {
      name: `${namespaceNames.argocd}`,
      namespace: namespaceNames.argocd,
    },
  },
  { provider: nameSpacesProvider }
);

export const certManagerNamespace = new Namespace(
  namespaceNames.certManager,
  {
    metadata: {
      name: `${namespaceNames.certManager}`,
      namespace: namespaceNames.certManager,
    },
  },
  { provider: nameSpacesProvider }
);

// export const infrastructureNamespace = new Namespace(
//   namespaceNames.argocd,
//   {
//     metadata: {
//       name: `${namespaceNames.argocd}`,
//       namespace: namespaceNames.argocd,
//     },
//   },
//   { provider: nameSpacesProvider }
// );

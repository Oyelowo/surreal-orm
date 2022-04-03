import * as k8s from "@pulumi/kubernetes";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import * as kx from "@pulumi/kubernetesx";

import { nameSpacesProvider } from "./manifestsDirectory";

// export const devNamespaceName = devNamespace.metadata.name as unknown as string;
export const namespaceNames = {
  applications: "applications",
  argocd: "argocd",
  default: "default",
  // Default namespace that comes with the deployment
  kubeSystem: "kube-system",
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

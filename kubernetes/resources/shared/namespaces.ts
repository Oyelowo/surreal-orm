import * as k8s from "@pulumi/kubernetes";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import * as kx from "@pulumi/kubernetesx";

import {
  clusterSetupProvider,
  providerApplication,
  providerNameSpacesProvider,
} from "./cluster";

// export const devNamespaceName = devNamespace.metadata.name as unknown as string;
export const devNamespaceName = "development";
export const devNamespace = new Namespace(
  devNamespaceName,
  {
    metadata: { name: `${devNamespaceName}`, namespace: devNamespaceName },
  },
  { provider: providerNameSpacesProvider }
);

export const argocdNamespaceName = "argocd";
export const argocdNamespace = new Namespace(
  argocdNamespaceName,
  {
    metadata: {
      name: `${argocdNamespaceName}`,
      namespace: argocdNamespaceName,
    },
  },
  { provider: providerNameSpacesProvider }
);

export const clusterSetupNamespaceName = "clusterSetup";
export const clusterSetupNamespace = new Namespace(
  clusterSetupNamespaceName,
  {
    metadata: {
      name: `${clusterSetupNamespaceName}`,
      // namespace: clusterSetupNamespaceName,
    },
  },
  { provider: clusterSetupProvider }
);

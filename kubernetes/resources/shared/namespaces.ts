import { provider, providerNameSpace } from "./cluster";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { Namespace } from "@pulumi/kubernetes/core/v1";

// export const devNamespaceName = devNamespace.metadata.name as unknown as string;
export const devNamespaceName = "development";
export const devNamespace = new Namespace(
  devNamespaceName,
  {
    metadata: { name: `${devNamespaceName}`, namespace: devNamespaceName },
  },
  { provider: providerNameSpace }
);

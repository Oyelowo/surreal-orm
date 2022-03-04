import * as k8s from '@pulumi/kubernetes';
import { Namespace } from '@pulumi/kubernetes/core/v1';
import * as kx from '@pulumi/kubernetesx';

import { provider, providerNameSpace } from './cluster';

// export const devNamespaceName = devNamespace.metadata.name as unknown as string;
export const devNamespaceName = "development";
export const devNamespace = new Namespace(
  devNamespaceName,
  {
    metadata: { name: `${devNamespaceName}`, namespace: devNamespaceName },
  },
  { provider: providerNameSpace }
);

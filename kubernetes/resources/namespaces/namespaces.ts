import { namespaceNames } from './util';
import * as k8s from "@pulumi/kubernetes";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import { getNamespacesNamesDir } from "../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../shared/validations";
// import { createArgocdApplication } from "./createArgoApplication";


const { ENVIRONMENT } = getEnvironmentVariables();
// export const linkerdVizDir = getLinkerdVizDir(
//   getEnvironmentVariables().ENVIRONMENT
// );

export const namespacesProvider = new k8s.Provider("render-namespaces", {
  renderYamlToDirectory: getNamespacesNamesDir(ENVIRONMENT),
});



type Keys = keyof typeof namespaceNames;
export type NamespaceName = typeof namespaceNames[Keys];

export const resourceNamespaces = Object.entries(namespaceNames).map(([_k, namespace]) => {
  const resourceNamespace = new Namespace(
    namespace,
    {
      metadata: {
        name: namespace,
        namespace,
        labels: {
          "config.linkerd.io/admission-webhooks": namespace === "linkerd" ? "disabled" : ""
        },
        annotations: {
          "linkerd.io/inject": "enabled"
        }
      },
    },
    { provider: namespacesProvider }
  );
  return resourceNamespace
})



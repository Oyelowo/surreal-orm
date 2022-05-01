import * as k8s from "@pulumi/kubernetes";
import { Namespace } from "@pulumi/kubernetes/core/v1";
import { getGeneratedEnvManifestsDir, getRepoPathFromAbsolutePath } from "./manifestsDirectory";
import { getEnvironmentVariables } from "./validations";
import path from "path";
import { createArgocdApplication } from "./createArgoApplication";


const { ENVIRONMENT } = getEnvironmentVariables();

const MANIFESTS_BASE_DIR_FOR_ENV = getGeneratedEnvManifestsDir(ENVIRONMENT);
const namespacesDir = path.join(MANIFESTS_BASE_DIR_FOR_ENV, "namespaces")

export const nameSpacesProvider = new k8s.Provider("render-namespaces", {
  renderYamlToDirectory: namespacesDir,
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
        annotations: {
          "linkerd.io/inject": "enabled"
        }
      },
    },
    { provider: nameSpacesProvider }
  );
  return resourceNamespace
})


// App that deploys Linkerd2 resources themselves
/* Linkerd2 APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const Linkerd2Application = createArgocdApplication({
  metadata: { name: "namespace-names", namespace: namespaceNames.default },
  pathToAppManifests: getRepoPathFromAbsolutePath(namespacesDir),
});


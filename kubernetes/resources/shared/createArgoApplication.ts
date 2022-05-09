import { v4 as uuid } from "uuid";
import path from "path";
import { NamespaceName, namespaceNames } from "./../namespaces/util";
import {
  ResourceName,
  getResourceRelativePath,
  getResourceProvider,
  getArgocdChildrenResourcesProvider,
  getPathToResourceType,
  getArgocdChildrenApplicationsAbsolutePath,
  ResourceType,
  getRepoPathFromAbsolutePath,
  getArgocdParentApplicationsPath,
} from "./manifestsDirectory";
import { CustomResourceOptions } from "@pulumi/pulumi";
import * as argocd from "../../crd2pulumi/argocd";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { getEnvironmentVariables } from "./validations";
import { getSecretsForApp } from "../../scripts/secretsManagement/getSecretsForApp";
// import { argoAppsOfApp } from "../infrastructure/argocd";

// export const argocdApplicationsInfraProvider = new k8s.Provider(
//   argocdApplicationsName + "infrastructure",
//   {
//     renderYamlToDirectory: getArgocdInfraApplicationsDir(
//       getEnvironmentVariables().ENVIRONMENT
//     ),
//   }
// );

// export const argocdApplicationsServicesProvider = new k8s.Provider(
//   argocdApplicationsName + "services",
//   {
//     renderYamlToDirectory: getArgocdServicesApplicationsDir(
//       getEnvironmentVariables().ENVIRONMENT
//     ),
//   }
// );

// export const argocdApplicationsNamespaceNamesProvider = new k8s.Provider(
//   argocdApplicationsName + "namespacesnames",
//   {
//     renderYamlToDirectory: getNamespacesNamesArgoAppDir(
//       getEnvironmentVariables().ENVIRONMENT
//     ),
//   }
// );

// export const argoAppsParentsProvider = new k8s.Provider(
//   argocdApplicationsName + "argoAppsParents",
//   {
//     renderYamlToDirectory: getArgoAppsParentsDir(
//       getEnvironmentVariables().ENVIRONMENT
//     ),
//   }
// );

// const providers: Record<ResourceType, k8s.Provider> = {
//   infrastructure: argocdApplicationsInfraProvider,
//   services: argocdApplicationsServicesProvider,
//   namespaces: argocdApplicationsNamespaceNamesProvider,
//   argo_applications_parents: argoAppsParentsProvider,
// };
// const getArgoAppDir = (resourceType: ResourceType) => {
//   return providers[resourceType];
// };
type Metadata = {};

type ArgocdApplicationProps = {
  name: string;
  labels?: Record<string, string>;
  namespace: NamespaceName;
  sourcePath: string;
  provider: k8s.Provider;
};

function createArgocdApplication({
  name,
  namespace,
  labels,
  provider,
  sourcePath,
}: ArgocdApplicationProps) {
  const metadata = {
    name,
    namespace,
    labels,
  };
  const argocdApplication = new argocd.argoproj.v1alpha1.Application(
    name,
    {
      metadata: {
        name: name,
        namespace: namespaceNames.argocd,
        annotations: {
          finalizers: ["resources-finalizer.argocd.argoproj.io"] as any,
          // Maybe use? argocd.argoproj.io / hook: PreSync
        },
      },
      spec: {
        project: "default",
        destination: {
          server: "https://kubernetes.default.svc",
          namespace: metadata.namespace,
        },
        source: {
          repoURL: "https://github.com/Oyelowo/modern-distributed-app-template",
          path: sourcePath,
          targetRevision: "HEAD",
          directory: {
            recurse: true,
          },
        },
        // syncPolicy: {
        //   automated: {
        //     prune: true,
        //     selfHeal: true,
        //   },
        // },
      },
    },
    {
      provider,
    }
  );

  return argocdApplication;
}

type ArgocdChildrenApplicationProps = {
  resourceName: ResourceName;
  labels?: Record<string, string>;
  namespace: NamespaceName;
  opts?: CustomResourceOptions | undefined;
  isParentApp?: boolean;
};

const { ENVIRONMENT } = getEnvironmentVariables();

export function createArgocdChildrenApplication({
  resourceName,
  namespace,
  labels,
}: ArgocdChildrenApplicationProps): argocd.argoproj.v1alpha1.Application {
  return createArgocdApplication({
    name: resourceName,
    namespace,
    labels,
    sourcePath: getResourceRelativePath(resourceName, ENVIRONMENT),
    provider: getArgocdChildrenResourcesProvider(resourceName, ENVIRONMENT),
  });
}

type ArgocdParentsApplicationProps = {
  name: string;
  resourceType: ResourceType;
  labels?: Record<string, string>;
  namespace: NamespaceName;
  opts?: CustomResourceOptions | undefined;
  isParentApp?: boolean;
};



export const providerArgoCDApplicationsParent = new k8s.Provider(
  `argocd-parents-applications-${uuid()}`,
  {
    renderYamlToDirectory: getArgocdParentApplicationsPath(ENVIRONMENT),
  }
);

export function createArgocdParentsApplication({
  namespace,
  resourceType,
  name,
  labels,
}: ArgocdParentsApplicationProps): argocd.argoproj.v1alpha1.Application {
  return createArgocdApplication({
    name,
    namespace,
    labels,
    sourcePath: getRepoPathFromAbsolutePath(
      getArgocdChildrenApplicationsAbsolutePath(resourceType, ENVIRONMENT)
    ),
    provider: providerArgoCDApplicationsParent,
  });
}
const metadata: Omit<Metadata, "argoApplicationName" | "resourceType"> = {
  name: "argocd-applications-secret",
  namespace: namespaceNames.argocd,
  labels: {
    "argocd.argoproj.io/secret-type": "repository",
  },
};


const secrets = getSecretsForApp("argocd", ENVIRONMENT);

export const argoCDApplicationsSecret = new kx.Secret(
  `argocd-secret`,
  {
    stringData: {
      ...secrets,
    },
    metadata,
  },
  { provider: providerArgoCDApplicationsParent }
);

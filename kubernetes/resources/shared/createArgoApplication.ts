import { NamespaceName, namespaceNames } from "./../namespaces/util";
import {
  ResourceName,
  getResourceRelativePath,
  getArgoResourceProvider,
} from "./manifestsDirectory";
import { CustomResourceOptions } from "@pulumi/pulumi";
import * as argocd from "../../crd2pulumi/argocd";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { getEnvironmentVariables } from "./validations";
import { getSecretsForApp } from "../../scripts/secretsManagement/getSecretsForApp";

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

type Props = {
  name?: ResourceName;
  labels?: Record<string, string>;
  namespace: NamespaceName;
  // resourceType: ResourceType;
  resourceName: ResourceName;
  // environment: Environment,
  opts?: CustomResourceOptions | undefined;
};

export function createArgocdApplication({
  name,
  namespace,
  labels,
  resourceName,
  // resourceType,
  // environment,
  opts,
}: Props) {
  const metadata = {
    name,
    namespace,
    labels,
  };
  // const metadataValues: Metadata = {
  //   name: metadata.name,
  //   namespace: metadata.namespace,
  //   labels: {
  //     "argocd.argoproj.io/secret-type": "repository",
  //     ...metadata.labels,
  //   },
  // };
  const { ENVIRONMENT } = getEnvironmentVariables();
  const argocdApplication = new argocd.argoproj.v1alpha1.Application(
    resourceName,
    {
      // apiVersion: "argoproj.io/v1alpha1",
      // kind: "Application",
      metadata: {
        name: resourceName,
        // name: metadata.name,
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
          // name: metadata.name,
          namespace: metadata.namespace,
        },
        source: {
          repoURL: "https://github.com/Oyelowo/modern-distributed-app-template",
          // path: pathToAppManifests,
          path: getResourceRelativePath(resourceName, ENVIRONMENT),
          //   path: "kubernetes/manifests/generated",
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
      provider: getArgoResourceProvider(resourceName, ENVIRONMENT),
      // provider: getArgoAppDir(metadata.resourceType),
      ...opts,
    }
  );

  return argocdApplication;
}

// const providersEntries = Object.entries(providers) as [
//   ResourceType,
//   k8s.Provider
// ][];

// export const secrets = providersEntries.map(([resourceType, provider]) => {
//   const metadata: Omit<Metadata, "argoApplicationName" | "resourceType"> = {
//     name: "argocd-applications-secret",
//     namespace: namespaceNames.argocd,
//     labels: {
//       "argocd.argoproj.io/secret-type": "repository",
//     },
//   };

//   /* SECRET */
//   const secrets = getSecretsForApp("argocd");

//   const argoCDApplicationsSecret = new kx.Secret(
//     `argocd-secret` + resourceType,
//     // `${resourceName}-secret`,
//     {
//       stringData: {
//         ...secrets,
//       },
//       metadata,
//     },
//     { provider }
//   );

//   return argoCDApplicationsSecret;
// });

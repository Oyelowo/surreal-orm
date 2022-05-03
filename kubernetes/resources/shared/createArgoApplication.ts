import { namespacesProvider } from "./../namespaces/namespaces";
import {
  argocdApplicationsName,
  argoApplicationsNames,
  ArgoApplicationName,
  ResourceType,
  getArgocdInfraApplicationsDir,
  getArgocdServicesApplicationsDir,
  getNamespacesNamesArgoAppDir,
  getArgoAppsParentsDir,
} from "./manifestsDirectory";
import { CustomResourceOptions, Resource } from "@pulumi/pulumi";
import * as argocd from "../../crd2pulumi/argocd";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { namespaceNames } from "../namespaces/util";
import { getEnvironmentVariables } from "./validations";
import { getSecretsForApp } from "../../scripts/secretsManagement/getSecretsForApp";

export const argocdApplicationsInfraProvider = new k8s.Provider(
  argocdApplicationsName + "infrastructure",
  {
    renderYamlToDirectory: getArgocdInfraApplicationsDir(
      getEnvironmentVariables().ENVIRONMENT
    ),
  }
);

export const argocdApplicationsServicesProvider = new k8s.Provider(
  argocdApplicationsName + "services",
  {
    renderYamlToDirectory: getArgocdServicesApplicationsDir(
      getEnvironmentVariables().ENVIRONMENT
    ),
  }
);

export const argocdApplicationsNamespaceNamesProvider = new k8s.Provider(
  argocdApplicationsName + "namespacesnames",
  {
    renderYamlToDirectory: getNamespacesNamesArgoAppDir(
      getEnvironmentVariables().ENVIRONMENT
    ),
  }
);

export const argoAppsParentsProvider = new k8s.Provider(
  argocdApplicationsName + "argoAppsParents",
  {
    renderYamlToDirectory: getArgoAppsParentsDir(
      getEnvironmentVariables().ENVIRONMENT
    ),
  }
);

const providers: Record<ResourceType, k8s.Provider> = {
  infrastructure: argocdApplicationsInfraProvider,
  services: argocdApplicationsServicesProvider,
  namespaces: argocdApplicationsNamespaceNamesProvider,
  "argo_applications_parents": argoAppsParentsProvider,
};
const getArgoAppDir = (resourceType: ResourceType) => {
  return providers[resourceType];
};
type Metadata = {
  name: string;
  namespace: string;
  // TODO: Consider removing this or replace the name above with it
  // argoApplicationName: ArgoApplicationName;
  resourceType: ResourceType;
  labels?: Record<string, string>;
};

type Props = {
  pathToAppManifests: string;
  metadata: Metadata;
  // provider: ProviderResource;
  opts?: CustomResourceOptions | undefined;
};

export function createArgocdApplication({
  metadata,
  pathToAppManifests,
  opts,
}: Props) {
  // const metadataValues: Metadata = {
  //   name: metadata.name,
  //   namespace: metadata.namespace,
  //   labels: {
  //     "argocd.argoproj.io/secret-type": "repository",
  //     ...metadata.labels,
  //   },
  // };
  const argocdApplication = new argocd.argoproj.v1alpha1.Application(
    metadata.name,
    {
      // apiVersion: "argoproj.io/v1alpha1",
      // kind: "Application",
      metadata: {
        name: metadata.name,
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
          path: pathToAppManifests,
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
      provider: getArgoAppDir(metadata.resourceType),
      ...opts,
    }
  );

  return argocdApplication;
}

const providersEntries = Object.entries(providers) as [
  ResourceType,
  k8s.Provider
][];

export const secrets = providersEntries.map(([resourceType, provider]) => {
  const metadata: Omit<Metadata, "argoApplicationName" | "resourceType"> = {
    name: "argocd-applications-secret",
    namespace: namespaceNames.argocd,
    labels: {
      "argocd.argoproj.io/secret-type": "repository",
    },
  };

  /* SECRET */
  const secrets = getSecretsForApp("argocd");

  const argoCDApplicationsSecret = new kx.Secret(
    `argocd-secret` + resourceType,
    // `${resourceName}-secret`,
    {
      stringData: {
        ...secrets,
      },
      metadata,
    },
    { provider }
  );

  return argoCDApplicationsSecret;
});

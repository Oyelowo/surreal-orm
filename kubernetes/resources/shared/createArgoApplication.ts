import { getPathToNonApplicationDir } from "./manifestsDirectory";
import { CustomResourceOptions, Resource } from "@pulumi/pulumi";
import * as argocd from "../../crd2pulumi/argocd";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { getSecretForApp } from "../../secretsManagement";
import { namespaceNames } from "./namespaces";
import { getEnvironmentVariables } from "./validations";

export const argocdApplicationsProvider = new k8s.Provider("argocd-applications", {
  renderYamlToDirectory: getPathToNonApplicationDir(
    "argocd-applications",
    getEnvironmentVariables().ENVIRONMENT
  ),
});

type Metadata = {
  name: string;
  namespace: string;
  labels?: Record<string, string>;
};

type Props = {
  pathToAppManifests: string;
  metadata: Metadata;
  // provider: ProviderResource;
  opts?: CustomResourceOptions | undefined;
};

export function createArgocdApplication({ metadata, pathToAppManifests, opts }: Props) {
  const metadataValues: Metadata = {
    name: metadata.name,
    namespace: metadata.namespace,
    labels: {
      "argocd.argoproj.io/secret-type": "repository",
      ...metadata.labels,
    },
  };
  const argocdApplication = new argocd.argoproj.v1alpha1.Application(
    metadata.name,
    {
      apiVersion: "argoproj.io/v1alpha1",
      // metadata: metadataValues,
      metadata: {
        name: metadata.name,
        namespace: "argocd",
      },
      spec: {
        project: "default",
        destination: {
          server: "https://kubernetes.default.svc",
          name: metadata.name,
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
        syncPolicy: {
          automated: {
            prune: true,
            selfHeal: true,
          },
        },
      },
    },
    { provider: argocdApplicationsProvider, ...opts }
  );

  return argocdApplication;
}

const metadata: Metadata = {
  name: "argocd-applications-secret",
  namespace: namespaceNames.argocd,
  labels: {
    "argocd.argoproj.io/secret-type": "repository",
  },
};

/* SECRET */
const secrets = getSecretForApp("argocd");

export const argoCDApplicationsSecret = new kx.Secret(
  `argocd-secret`,
  // `${resourceName}-secret`,
  {
    stringData: {
      ...secrets,
    },
    metadata,
  },
  { provider: argocdApplicationsProvider }
);

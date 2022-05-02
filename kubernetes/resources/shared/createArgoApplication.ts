import { getArgocdApplicationsDir, argocdApplicationName } from "./manifestsDirectory";
import { CustomResourceOptions, Resource } from "@pulumi/pulumi";
import * as argocd from "../../crd2pulumi/argocd";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { namespaceNames } from "../namespaces/util";
import { getEnvironmentVariables } from "./validations";
import { getSecretsForApp } from "../../scripts/secretsManagement/getSecretsForApp";

export const argocdApplicationsProvider = new k8s.Provider(argocdApplicationName, {
  renderYamlToDirectory: getArgocdApplicationsDir(getEnvironmentVariables().ENVIRONMENT),
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
        namespace: namespaceNames.argocd,
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
const secrets = getSecretsForApp("argocd");

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

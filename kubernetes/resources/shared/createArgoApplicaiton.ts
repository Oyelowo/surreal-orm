import { ProviderResource } from "@pulumi/pulumi";
import * as argocd from "../../crd2pulumi/argocd";

type Metadata = {
  name: string;
  namespace: string;
  labels?: Record<string, string>;
};

type Props = {
  pathToAppManifests: string;
  metadata: Metadata;
  provider: ProviderResource;
};

export function createArgocdApplication({ metadata, pathToAppManifests, provider }: Props) {
  const metadataValues: Metadata = {
    name: metadata.name,
    namespace: metadata.namespace,
    labels: {
      "argocd.argoproj.io/secret-type": "repository",
      ...metadata.labels,
    },
  };
  const argocdApplication = new argocd.argoproj.v1alpha1.Application(
    "argocd-oyelowo-applications",
    {
      apiVersion: "argoproj.io/v1alpha1",
      metadata: metadataValues,
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
    { provider }
  );

  return argocdApplication;
}

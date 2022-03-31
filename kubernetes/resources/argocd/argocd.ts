import * as k8s from "@pulumi/kubernetes";
import {
  applicationsDirectory,
  argocdDirectory,
} from "../shared/manifestsDirectory";
import { namespaceNames } from "../shared/namespaces";
import { ArgocdHelmValuesBitnami } from "../shared/types/helm-charts/argocdHelmValuesBitnami";
import { DeepPartial, RecursivePartial } from "../shared/types/own-types";
import * as kx from "@pulumi/kubernetesx";
import * as path from "path";
import * as argocd from "../../crd2pulumi/argocd";
import { getSecretForApp } from "../../secretsManagement";


const secrets = getSecretForApp("argocd")

type Metadata = {
  name: string;
  namespace: string;
  labels: {
    "argocd.argoproj.io/secret-type": "repository";
  };
};
const metadata: Metadata = {
  name: "argocd-application-secret",
  namespace: namespaceNames.argocd,
  labels: {
    "argocd.argoproj.io/secret-type": "repository",
  },
};
const resourceName = metadata.name;

export const argocdOyelowoApplications =
  new argocd.argoproj.v1alpha1.Application(
    "argocd-oyelowo-applications",
    {
      apiVersion: "argoproj.io/v1alpha1",
      metadata,
      spec: {
        project: "oyelowo-project",
        destination: {
          server: "https://kubernetes.default.svc",
          namespace: namespaceNames.applications,
          name: "oyelowo-applications",
        },
        source: {
          repoURL: "https://github.com/Oyelowo/modern-distributed-app-template",
          path: "kubernetes/manifests/generated",
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
    { provider: argocdDirectory }
  );

export const argoCDApplicationsSecret = new kx.Secret(
  `${resourceName}-secret`,
  {
    stringData: {
      type: "git",
      url: "https://github.com/Oyelowo/modern-distributed-app-template",
      username: "Oyelowo",
      password: "my-password-or-personal-access-token",
      
    },
    metadata,
  },
  { provider: applicationsDirectory }
);

// export const argoApplicationSecret = new k8s.

const argocdValues: DeepPartial<ArgocdHelmValuesBitnami> = {
  // fullnameOverride: "argocd",
  // clusterDomain: "https:kubernetes.default.svc",
  // repoServer: {},
  // global: {},
  // config: {
  //   secret: {
  //     githubSecret: "",
  //     repositoryCredentials: {}
  //   }
  // },
  // server: {
  //   url: "https:kubernetes.default.svc",
  // },
  // redis: {
  //   enabled: false,
  // },
  // externalRedis: {
  //   host: "",
  //   password: "",
  //   port: 33,
  //   existingSecretPasswordKey: "",
  //   existingSecret: "",
  // },
  config: {
    secret: {
      argocdServerAdminPassword: "",
    },
  },
  redis: {
    architecture: "standalone",
    auth: {
      enabled: true,
      existingSecret: "wert",
      existingSecretPasswordKey: "1234",
    },
  },
  server: {
    ingress: {
      enabled: true,
      hostname: "194-195-247-46.ip.linodeusercontent.com",
      path: "/tools/argocd",
      pathType: "prefix",
      ingressClassName: "nginx",
    },
  },
  dex: {
    enabled: false,
  },
};

// `http://${name}.${namespace}:${port}`;
export const argocdHelm = new k8s.helm.v3.Chart(
  "argo-cd",
  {
    chart: "argo-cd",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "2.0.4",
    values: argocdValues,
    namespace: namespaceNames.argocd,
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: argocdDirectory }
  // { provider }
);

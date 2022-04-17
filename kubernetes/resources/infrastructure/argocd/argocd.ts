import * as k8s from "@pulumi/kubernetes";
import { getArgocdControllerDir, getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { namespaceNames } from "../../shared/namespaces";
import { ArgocdHelmValuesBitnami } from "../../shared/types/helm-charts/argocdHelmValuesBitnami";
import { ArgocdHelmValuesArgo } from "../../shared/types/helm-charts/argocdHelmValuesArgo";
import { DeepPartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
const argocdControllerDir = getArgocdControllerDir(ENVIRONMENT);

type Metadata = {
  name: string;
  namespace: string;
};
const metadata: Metadata = {
  name: "argocd-application",
  namespace: namespaceNames.argocd,
};

const resourceName = metadata.name;

// App that deploys argocd resources themselves
/* ARGOCD APPLICATION ITSELF RESPONSIBLE FOR DECLARATIVELY DEPLOYING ARGO CONTROLLER RESOURCES */
export const argocdApplication = createArgocdApplication({
  metadata,
  pathToAppManifests: getRepoPathFromAbsolutePath(argocdControllerDir),
});

export const argocdControllerProvider = new k8s.Provider(argocdControllerDir, {
  renderYamlToDirectory: argocdControllerDir,
});

// export const argoApplicationSecret = new k8s.

/* const argocdValuesOld: DeepPartial<ArgocdHelmValuesBitnami> = {
  fullnameOverride: "argocd",
  // global:{

  // },
  // TODO:,
  config: { secret: { argocdServerAdminPassword: "lowo" } },
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
  // config: {
  //   secret: {
  //     argocdServerAdminPassword: "",
  //   },
  // },
  redis: {
    architecture: "standalone",
    auth: {
      enabled: true,
      // existingSecretPasswordKey: "1234",
      // existingSecret: "wert" ,
    },
  },
  server: {
    ingress: {
      enabled: true,
      hostname: "172.104.255.25",
      path: "/tools/argocd",
      pathType: "Prefix" as "Exact" | "ImplementationSpecific" | "Prefix",
      ingressClassName: "nginx",
      secrets: []
    },
  },
  dex: {
    enabled: false,
  },
}; */

const argocdValues: DeepPartial<ArgocdHelmValuesArgo> = {
  fullnameOverride: "argocd",
  configs: {
    secret: {
      argocdServerAdminPassword: "lowo"
    }
  }
  ,
  dex: {
    enabled: false
  },
  redis: {
    enabled: true
  },
  notifications: {
    enabled: true,
    secret: {
      create: false,
      // items: {
      //   "name": "ererer"
      // }
    }
  }
  // redis: {

  // }
};
// `http://${name}.${namespace}:${port}`;
export const argocdHelm = new k8s.helm.v3.Chart(
  "argocd",
  {
    chart: "argo-cd",
    fetchOpts: {
      // repo: "https://charts.bitnami.com/bitnami",
      repo: "https://argoproj.github.io/argo-helm",
    },
    version: "4.5.3",
    // version: "3.1.12",
    values: argocdValues,
    namespace: namespaceNames.argocd,
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: argocdControllerProvider }
  // { provider }
);

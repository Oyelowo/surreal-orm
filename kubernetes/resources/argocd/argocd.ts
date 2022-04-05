import * as k8s from "@pulumi/kubernetes";
import { getPathToNonApplicationDir } from "../shared/manifestsDirectory";
import { createArgocdApplication } from "../shared/createArgoApplication";
import { namespaceNames } from "../shared/namespaces";
import { ArgocdHelmValuesBitnami } from "../shared/types/helm-charts/argocdHelmValuesBitnami";
import { DeepPartial, RecursivePartial } from "../shared/types/own-types";
import * as kx from "@pulumi/kubernetesx";
import * as path from "path";
import * as argocd from "../../crd2pulumi/argocd";
import { getSecretForApp } from "../../secretsManagement";
import { getEnvironmentVariables } from "../shared/validations";

const argocdControllerDir = getPathToNonApplicationDir("argocd-controller", getEnvironmentVariables().ENVIRONMENT);

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
  pathToAppManifests: argocdControllerDir,
});

export const argocdControllerProvider = new k8s.Provider(argocdControllerDir, {
  renderYamlToDirectory: argocdControllerDir,
});

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
      hostname: "194-195-247-46.ip.linodeusercontent.com",
      path: "/tools/argocd",
      pathType: "Prefix" as "Exact" | "ImplementationSpecific" | "Prefix",
      ingressClassName: "nginx",
    },
  },
  dex: {
    enabled: false,
  },
};

// `http://${name}.${namespace}:${port}`;
export const argocdHelm = new k8s.helm.v3.Chart(
  "argocd",
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
  { provider: argocdControllerProvider }
  // { provider }
);

import { DOMAIN_NAME_BASE, DOMAIN_NAME_SUB_ARGOCD } from './../ingress-controller/constant';
import { annotations } from './../ingress-controller/ingressRules';
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


export const argocdControllerProvider = new k8s.Provider(argocdControllerDir, {
  renderYamlToDirectory: argocdControllerDir,
});


const argocdValuesOld: DeepPartial<ArgocdHelmValuesBitnami> = {
  // fullnameOverride: "argocd",
  // global:{

  // },
  // TODO:,
  // controller: { serviceAccount: { create: false } },
  config: { secret: { create: true, argocdServerAdminPassword: "oyelowo", } },
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
  // rbac: { create: true },
  // redis: {
  //   enabled: true,
  //   architecture: "standalone",
  //   auth: {
  //     enabled: true,
  //     // existingSecretPasswordKey: "1234",
  //     // existingSecret: "wert" ,
  //   },
  // },
  server: {
    ingress: {
      enabled: true,
      hostname: DOMAIN_NAME_SUB_ARGOCD,
      annotations: annotations,
      pathType: "Prefix" as "Exact" | "ImplementationSpecific" | "Prefix",
      ingressClassName: "nginx",
      tls: true

    },
    // Ingress-controller already handles TLS. Argocd does too which causes collision. Disable argo from doing that
    // https://stackoverflow.com/questions/49856754/nginx-ingress-too-many-redirects-when-force-ssl-is-enabled
    extraArgs: ["--insecure"] as any[]
  },
  dex: {
    enabled: false,
  },
};


export const argocdHelm = new k8s.helm.v3.Chart(
  "argocd",
  {
    chart: "argo-cd",
    fetchOpts: {
      repo: "https://charts.bitnami.com/bitnami",
    },
    version: "3.1.12",
    values: argocdValuesOld,
    namespace: namespaceNames.argocd,
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: argocdControllerProvider }
);

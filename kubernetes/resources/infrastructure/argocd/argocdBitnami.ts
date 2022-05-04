import { DOMAIN_NAME_SUB_ARGOCD } from '../ingress/constant';
import { annotations, INGRESS_CLASSNAME_NGINX } from '../ingress/ingressRules';
import * as k8s from "@pulumi/kubernetes";
import { namespaceNames } from "../../namespaces/util";
import { ArgocdHelmValuesBitnami } from "../../shared/types/helm-charts/argocdHelmValuesBitnami";
import { DeepPartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";
import { argocdProperties } from './settings';


// TODO: Use this everywhere
const STORAGE_CLASS = "linode-block-storage-retain"
const argocdValuesOld: DeepPartial<ArgocdHelmValuesBitnami> = {
  config: { secret: { create: true, argocdServerAdminPassword: "oyelowo", } },
  global: {
    storageClass:
      getEnvironmentVariables().ENVIRONMENT === "local" ? "" : STORAGE_CLASS,
  },
  server: {
    ingress: {
      enabled: true,
      hostname: DOMAIN_NAME_SUB_ARGOCD,
      annotations: annotations,
      pathType: "Prefix" as "Exact" | "ImplementationSpecific" | "Prefix",
      ingressClassName: INGRESS_CLASSNAME_NGINX,
      tls: true,
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
    version: "3.1.14",
    values: argocdValuesOld,
    namespace: namespaceNames.argocd,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: argocdProperties.provider }
);

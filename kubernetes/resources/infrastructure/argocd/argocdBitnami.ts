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
  // server: {
  //   ingress: {
  //     enabled: true,
  //     hostname: "172.104.255.25",
  //     path: "/tools/argocd",
  //     pathType: "Prefix" as "Exact" | "ImplementationSpecific" | "Prefix",
  //     ingressClassName: "nginx",
  //     secrets: []
  //   },
  // },
  dex: {
    enabled: false,
  },
};

import bcrypt from "bcrypt"

const saltRounds = 10;
const myPlaintextPassword = 'oyelowo';
const hash = bcrypt.hashSync(myPlaintextPassword, saltRounds);
const argocdValues: DeepPartial<ArgocdHelmValuesArgo> = {
  fullnameOverride: "argocd",
  server: {

  },
  configs: {
    secret: {
      // createSecret: false,
      argocdServerAdminPassword: hash,
      // argocdServerAdminPassword: "lowoo",
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
      create: true,
      items: {
        "name": "ererer"
      }
    }
  }
  // redis: {

  // }
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

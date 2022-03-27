import * as k8s from "@pulumi/kubernetes";

import { clusterSetupProvider, providerApplication } from "../shared/cluster";
import { devNamespaceName } from "../shared/namespaces";
import { SealedSecretsHelmValuesBitnami } from "../shared/sealedSecretsHelmValuesBitnami";
import { DeepPartial, RecursivePartial } from "../shared/types";

const sealedSecretsValues: DeepPartial<SealedSecretsHelmValuesBitnami> = {
  // nameOverride: "mongodb-graphql",

  /* 
  NOTE: the helm chart by default installs the controller with the name sealed-secrets, while the kubeseal command line interface (CLI) tries to access the controller with the name sealed-secrets-controller. You can explicitly pass --controller-name to the CLI:
kubeseal --controller-name sealed-secrets <args>
Alternatively, you can override fullnameOverride on the helm chart install.
  */
  fullnameOverride: "sealed-secrets-controller",
};

// `http://${name}.${namespace}:${port}`;
export const sealedSecret = new k8s.helm.v3.Chart(
  "sealed-secrets-controller",
  {
    chart: "sealed-secrets",
    fetchOpts: {
      repo: "https://bitnami-labs.github.io/sealed-secrets",
    },
    version: "2.1.4",
    values: sealedSecretsValues,
    namespace: "kube-system",
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: clusterSetupProvider }
  // { provider }
);

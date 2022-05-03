import { SealedSecretsHelmValuesBitnami } from "../../shared/types/helm-charts/sealedSecretsHelmValuesBitnami";
import * as k8s from "@pulumi/kubernetes";

import {
  getSealedSecretsControllerDir,
  sealedSecretsControllerName,
} from "../../shared/manifestsDirectory";
import { namespaceNames } from "../../namespaces/util";
import { DeepPartial, RecursivePartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";

export const sealedSecretsControllerDir = getSealedSecretsControllerDir(
  getEnvironmentVariables().ENVIRONMENT
);

const sealedSecretsProvider = new k8s.Provider(sealedSecretsControllerDir, {
  renderYamlToDirectory: sealedSecretsControllerDir,
});

const sealedSecretsValues: DeepPartial<SealedSecretsHelmValuesBitnami> = {
  // nameOverride: "mongodb-graphql",

  /* 
  NOTE: the helm chart by default installs the controller with the name sealed-secrets, while the kubeseal command line interface (CLI) tries to access the controller with the name sealed-secrets-controller. You can explicitly pass --controller-name to the CLI:
kubeseal --controller-name sealed-secrets <args>
Alternatively, you can override fullnameOverride on the helm chart install.
  */
  fullnameOverride: sealedSecretsControllerName,
  podAnnotations: {
    // ...getArgoAppSyncWaveAnnotation("sealed-secrets"),
  },
  // service: {
  //   annotations: {
  //     ...getArgoAppSyncWaveAnnotation("sealed-secrets")
  //   }
  // }
};

// `http://${name}.${namespace}:${port}`;
export const sealedSecret = new k8s.helm.v3.Chart(
  sealedSecretsControllerName,
  {
    chart: "sealed-secrets",
    fetchOpts: {
      repo: "https://bitnami-labs.github.io/sealed-secrets",
    },
    version: "2.1.7",
    values: sealedSecretsValues,
    namespace: namespaceNames.kubeSystem,
    // namespace: devNamespaceName,
    // By default Release resource will wait till all created resources
    // are available. Set this to true to skip waiting on resources being
    // available.
    skipAwait: false,
  },
  { provider: sealedSecretsProvider }
  // { provider }
);

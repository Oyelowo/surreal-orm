import * as k8s from '@pulumi/kubernetes';
import { namespaceNames } from '../../namespaces/util';
import { helmChartsInfo } from '../../shared/helmChartInfo';
import { SealedSecretsHelmValuesBitnami } from '../../shared/types/helm-charts/sealedSecretsHelmValuesBitnami';
import { DeepPartial } from '../../shared/types/own-types';
import { sealedSecretsResourceName, sealedSecretsProvider } from './settings';

const sealedSecretsValues: DeepPartial<SealedSecretsHelmValuesBitnami> = {
    // nameOverride: "mongodb-graphql",

    /*
  NOTE: the helm chart by default installs the controller with the name sealed-secrets, while the kubeseal command line interface (CLI) tries to access the controller with the name sealed-secrets-controller. You can explicitly pass --controller-name to the CLI:
kubeseal --controller-name sealed-secrets <args>
Alternatively, you can override fullnameOverride on the helm chart install.
  */
    fullnameOverride: sealedSecretsResourceName,
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
const {
    repo,
    sealedSecrets: { chart, version },
} = helmChartsInfo.sealedSecrets;
export const sealedSecret = new k8s.helm.v3.Chart(
    sealedSecretsResourceName,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: sealedSecretsValues,
        namespace: namespaceNames.kubeSystem,
        // namespace: devNamespaceName,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: sealedSecretsProvider }
);

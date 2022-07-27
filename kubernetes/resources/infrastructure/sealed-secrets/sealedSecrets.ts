import { ISealedsecretssealedsecrets } from '../../../generatedHelmChartsTsTypes/sealedSecretsSealedsecrets';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../namespaces/util';
import { helmChartsInfo } from '../../shared/helmChartInfo';
import { DeepPartial } from '../../types/own-types';
import { sealedSecretsResourceName, sealedSecretsProvider } from './settings';

const sealedSecretsValues: DeepPartial<ISealedsecretssealedsecrets> = {
    /*
  NOTE: the helm chart by default installs the controller with the name sealed-secrets, while the kubeseal command line interface (CLI) tries to access the controller with the name sealed-secrets-controller. You can explicitly pass --controller-name to the CLI:
kubeseal --controller-name sealed-secrets <args>
Alternatively, you can override fullnameOverride on the helm chart install.
  */
    fullnameOverride: sealedSecretsResourceName,
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        sealedSecrets: { chart, version },
    },
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
        namespace: namespaces.kubeSystem,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: sealedSecretsProvider }
);

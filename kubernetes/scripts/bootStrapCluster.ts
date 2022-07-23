import sh from 'shelljs';
import { clearPlainInputTsSecretFilesContents } from './secretsManagement/syncSecretsTsFiles';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { promptSecretsDeletionConfirmations } from './utils/promptSecretsDeletionConfirmations';
import { promptEnvironmentSelection } from './utils/shared';
import path from 'path';
import { namespaces } from '../resources/infrastructure/namespaces/util';
import { helmChartsInfo } from '../resources/shared/helmChartInfo';
import { Environment, ResourceName } from '../resources/types/own-types';
import { syncSecretsTsFiles } from './secretsManagement/syncSecretsTsFiles';
import { generateManifests } from './utils/kubeObject/generateManifests';
import { getImageTagsFromDir } from './utils/getImageTagsFromDir';
import _ from 'lodash';
import { KubeObject, TKubeObject } from './utils/kubeObject/kubeObject';

async function main() {
    const { environment } = await promptEnvironmentSelection();
    const kubeObject = new KubeObject(environment);
    await promptKubernetesClusterSwitch(environment);
    const { deletePlainSecretsInput, deleteUnsealedSecretManifestsOutput } = await promptSecretsDeletionConfirmations();

    await kubeObject.generateManifests();

    // Sync the TS config files where our gitignored secrets are stored locally
    syncSecretsTsFiles();

    await applySetupManifests(kubeObject);

    if (deletePlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (deleteUnsealedSecretManifestsOutput) {
        kubeObject.getOfAKind('Secret').forEach((o) => {
            sh.rm('-rf', o.path);
        });
    }
}

main();

async function applySetupManifests(kubeObject: KubeObject) {
    // const { getAllKubeManifestsInfo, syncManifestsInfo } = manifestsManager

    // # Apply namespace first
    applyResourceManifests('namespaces', kubeObject);

    // # Apply setups with sealed secret controller
    applyResourceManifests('sealed-secrets', kubeObject);

    const sealedSecretsName: ResourceName = 'sealed-secrets';
    // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
    sh.exec(`kubectl rollout status deployment/${sealedSecretsName} -n=${namespaces.kubeSystem}`);

    kubeObject.syncSealedSecretsWithPrompt();

    // # Apply setups with cert-manager controller
    applyResourceManifests('cert-manager', kubeObject);

    // # Wait for cert-manager and cert-manager-trust controllers to be in running phase so that we can use it to encrypt secrets
    const { certManager, certManagerTrust } = helmChartsInfo.jetstack.charts;
    sh.exec(`kubectl rollout status deployment/${certManager.chart} -n=${namespaces.certManager}`);
    sh.exec(`kubectl rollout status deployment/${certManagerTrust.chart} -n=${namespaces.certManager}`);

    // Reapply cert-manager in case something did not apply the first time e.g the cert-managerr-trust
    // needs to be ready for Bundle to be applied
    applyResourceManifests('cert-manager', kubeObject);

    // # Apply setups with linkerd controller
    applyResourceManifests('linkerd', kubeObject);
    applyResourceManifests('linkerd-viz', kubeObject);

    // applyResourceManifests('argocd', environment, allManifestsInfo);

    // sh.exec('kubectl -n argocd rollout restart deployment argocd-argo-cd-server');

    // applyResourceManifests('argocd-applications-parents', environment, allManifestsInfo);
    sh.exec(`skaffold dev --cleanup=false  --trigger="manual"  --no-prune=true --no-prune-children=true`);
}

function applyResourceManifests(resourceName: ResourceName, kubeObject: KubeObject) {
    const resource = kubeObject.getForApp(resourceName);

    // put crds and sealed secret resources first
    const manifestsToApply = _.sortBy(resource, [
        (m) => m.kind !== 'CustomResourceDefinition',
        (m) => m.kind !== 'SealedSecret',
    ]);

    // console.log('manifestsToApply', manifestsToApply);
    manifestsToApply.forEach((o) => sh.exec(`kubectl apply -f  ${o.path}`));
}

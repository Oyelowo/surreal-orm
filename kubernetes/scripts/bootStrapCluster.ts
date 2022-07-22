import sh from 'shelljs';
import { clearPlainInputTsSecretFilesContents } from './secretsManagement/syncSecretsTsFiles';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { promptSecretsDeletionConfirmations } from './utils/promptSecretsDeletionConfirmations';
import { promptEnvironmentSelection, getKubeManifestsPaths } from './utils/shared';

import { getAppResourceManifestsInfo, getAllKubeManifestsInfo, KubeObjectInfo } from './utils/shared';
import path from 'path';
import { namespaces } from '../resources/infrastructure/namespaces/util';
import { helmChartsInfo } from '../resources/shared/helmChartInfo';
import { Environment, ResourceName } from '../resources/types/own-types';
import { syncSecretsTsFiles } from './secretsManagement/syncSecretsTsFiles';
import { generateManifests } from './utils/generateManifests';
import { getImageTagsFromDir } from './utils/getImageTagsFromDir';
import { syncAppSealedSecrets } from './utils/syncAppsSecrets';
import _ from 'lodash';

async function main() {
    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);

    const { deletePlainSecretsInput, deleteUnsealedSecretManifestsOutput } = await promptSecretsDeletionConfirmations();

    const imageTags = await getImageTagsFromDir();

    await generateManifests({
        environment,
        imageTags,
        allManifestsInfo: getAllKubeManifestsInfo(environment),
    });

    syncSecretsTsFiles();

    // We want to refetch to make sure the info is upto data after generation
    const allManifestsInfo = getAllKubeManifestsInfo(environment);
    await applySetupManifests(environment, allManifestsInfo);

    if (deletePlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (deleteUnsealedSecretManifestsOutput) {
        const removeSecret = (path: string) => sh.rm('-rf', path);
        getKubeManifestsPaths({ kind: 'Secret', allManifestsInfo }).forEach(removeSecret);
    }
}

main();


async function applySetupManifests(environment: Environment, initialAllManifestsInfo: KubeObjectInfo[]) {
    /*
       This requires the above step with initial cluster setup making sure that the sealed secret controller is
       running in the cluster */

    // # Apply namespace first
    applyResourceManifests('namespaces', environment, initialAllManifestsInfo);

    // # Apply setups with sealed secret controller
    applyResourceManifests('sealed-secrets', environment, initialAllManifestsInfo);

    const sealedSecretsName: ResourceName = 'sealed-secrets';
    // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
    sh.exec(`kubectl rollout status deployment/${sealedSecretsName} -n=${namespaces.kubeSystem}`);

    // This generates sealed secrets, so, we would want updated version of all manifests
    await syncAppSealedSecrets(environment, initialAllManifestsInfo);

    const updatedeAllManifestsInfo = getAllKubeManifestsInfo(environment);
    // # Apply setups with cert-manager controller
    applyResourceManifests('cert-manager', environment, updatedeAllManifestsInfo);

    // # Wait for cert-manager and cert-manager-trust controllers to be in running phase so that we can use it to encrypt secrets
    const { certManager, certManagerTrust } = helmChartsInfo.jetstack.charts;
    sh.exec(`kubectl rollout status deployment/${certManager.chart} -n=${namespaces.certManager}`);
    sh.exec(`kubectl rollout status deployment/${certManagerTrust.chart} -n=${namespaces.certManager}`);

    // # Apply setups with linkerd controller
    applyResourceManifests('linkerd', environment, updatedeAllManifestsInfo);
    // applyResourceManifests('linkerd-viz', environment, upDatedeAllManifestsInfo);

    // applyResourceManifests('argocd', environment, upDatedeAllManifestsInfo);

    // sh.exec('kubectl -n argocd rollout restart deployment argocd-argo-cd-server');

    // applyResourceManifests('argocd-applications-parents', environment, upDatedeAllManifestsInfo);
}


function applyResourceManifests(
    resourceName: ResourceName,
    environment: Environment,
    allManifestsInfo: KubeObjectInfo[]
) {
    const manifestsInfo = getAppResourceManifestsInfo({ resourceName, environment, allManifestsInfo });

    // put crds and sealed secret resources first
    const manifestsToApply = _.sortBy(manifestsInfo, [
        (m) => m.kind !== 'CustomResourceDefinition',
        (m) => m.kind !== 'SealedSecret',
    ]);

    console.log('manifestsToApply', manifestsToApply);
    manifestsToApply.forEach((o) => sh.exec(`kubectl apply -f  ${o.path}`));
}

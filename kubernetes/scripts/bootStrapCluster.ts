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
import { ManifestsManager } from './utils/manifestsManager';

async function main() {
    // const { environment } = await promptEnvironmentSelection();
    const environment = 'local';
    // await promptKubernetesClusterSwitch(environment);

    // const { deletePlainSecretsInput, deleteUnsealedSecretManifestsOutput } = await promptSecretsDeletionConfirmations();
    const deletePlainSecretsInput = false;
    const deleteUnsealedSecretManifestsOutput = false;
    const { syncManifestsInfo, getAllKubeManifestsInfo } = new ManifestsManager('local');
    // const allManifestsInfo = manifests.getAllKubeManifestsInfo()

    const imageTags = await getImageTagsFromDir();

    await generateManifests({
        environment,
        imageTags,
        allManifestsInfo: getAllKubeManifestsInfo(),
    });

    // Sync the TS config files where our gitignored secrets are stored locally
    syncSecretsTsFiles();

    await applySetupManifests({
        environment,
        onNewManifestsGenerated: syncManifestsInfo,
        allManifestsInfo: getAllKubeManifestsInfo(),
    });

    if (deletePlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (deleteUnsealedSecretManifestsOutput) {
        const removeSecret = (path: string) => sh.rm('-rf', path);
        getKubeManifestsPaths({ kind: 'Secret', allManifestsInfo: getAllKubeManifestsInfo() }).forEach(
            removeSecret
        );
    }
}

main();

type Props = {
    environment: Environment;
    onNewManifestsGenerated: () => void;
    allManifestsInfo: KubeObjectInfo[];
};

async function applySetupManifests({
    environment,
    allManifestsInfo,
    onNewManifestsGenerated,
}: Props) {
    // const { getAllKubeManifestsInfo, syncManifestsInfo } = manifestsManager

    // # Apply namespace first
    applyResourceManifests('namespaces', environment, allManifestsInfo);

    // # Apply setups with sealed secret controller
    applyResourceManifests('sealed-secrets', environment, allManifestsInfo);

    const sealedSecretsName: ResourceName = 'sealed-secrets';
    // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
    sh.exec(`kubectl rollout status deployment/${sealedSecretsName} -n=${namespaces.kubeSystem}`);

    // This generates sealed secrets, so, we would want updated version of all manifests
    await syncAppSealedSecrets(environment, allManifestsInfo);

    // Regenerates/Syncs manifests info after sealed secrets manifests have been generated
    onNewManifestsGenerated();

    // # Apply setups with cert-manager controller
    applyResourceManifests('cert-manager', environment, allManifestsInfo);

    // # Wait for cert-manager and cert-manager-trust controllers to be in running phase so that we can use it to encrypt secrets
    const { certManager, certManagerTrust } = helmChartsInfo.jetstack.charts;
    sh.exec(`kubectl rollout status deployment/${certManager.chart} -n=${namespaces.certManager}`);
    sh.exec(`kubectl rollout status deployment/${certManagerTrust.chart} -n=${namespaces.certManager}`);

    // Reapply cert-manager in case something did not apply the first time e.g the cert-managerr-trust
    // needs to be ready for Bundle to be applied
    applyResourceManifests('cert-manager', environment, allManifestsInfo);

    // # Apply setups with linkerd controller
    applyResourceManifests('linkerd', environment, allManifestsInfo);
    applyResourceManifests('linkerd-viz', environment, allManifestsInfo);

    // applyResourceManifests('argocd', environment, allManifestsInfo);

    // sh.exec('kubectl -n argocd rollout restart deployment argocd-argo-cd-server');

    // applyResourceManifests('argocd-applications-parents', environment, allManifestsInfo);
    sh.exec(`skaffold dev --cleanup=false  --trigger="manual"  --no-prune=true --no-prune-children=true`);
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

    // console.log('manifestsToApply', manifestsToApply);
    manifestsToApply.forEach((o) => sh.exec(`kubectl apply -f  ${o.path}`));
}

#!/usr/bin/env ts-node

import path from 'path';
import sh from 'shelljs';
import { namespaces } from '../../resources/infrastructure/namespaces/util';
import { helmChartsInfo } from '../../resources/shared/helmChartInfo';
import { getResourceAbsolutePath } from '../../resources/shared/manifestsDirectory';
import { Environment, ResourceName } from '../../resources/types/own-types';
import { syncSecretsTsFiles } from '../secretsManagement/syncSecretsTsFiles';
import { generateAllSealedSecrets } from './sealed-secrets/generateAllSealedSecrets';
import { generateManifests } from './generateManifests';
import { getImageTagsFromDir } from './getImageTagsFromDir';
import { syncAppSealedSecrets } from './syncAppsSecrets';

export async function bootstrapCluster(environment: Environment) {
    const imageTags = await getImageTagsFromDir();

    await generateManifests({
        environment,
        imageTags,
    });

    syncAppSealedSecrets(environment)

    syncSecretsTsFiles();

    /*
       This requires the above step with initial cluster setup making sure that the sealed secret controller is
       running in the cluster */

    // # Apply namespace first
    applyResourceManifests('namespaces', environment);

    // # Apply setups with sealed secret controller
    applyResourceManifests('sealed-secrets', environment);

    const sealedSecretsName: ResourceName = 'sealed-secrets';
    // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
    sh.exec(`kubectl rollout status deployment/${sealedSecretsName} -n=${namespaces.kubeSystem}`);

    // # Apply setups with cert-manager controller
    applyResourceManifests('cert-manager', environment);

    // # Wait for cert-manager and cert-manager-trust controllers to be in running phase so that we can use it to encrypt secrets
    const { certManager, certManagerTrust } = helmChartsInfo.jetstack.charts;
    sh.exec(`kubectl rollout status deployment/${certManager.chart} -n=${namespaces.certManager}`);
    sh.exec(`kubectl rollout status deployment/${certManagerTrust.chart} -n=${namespaces.certManager}`);

    // # Apply setups with linkerd controller
    applyResourceManifests('linkerd', environment);
    applyResourceManifests('linkerd-viz', environment);

    await generateAllSealedSecrets({
        environment,
    });

    applyResourceManifests('argocd', environment);

    sh.exec('kubectl -n argocd rollout restart deployment argocd-argo-cd-server');

    applyResourceManifests('argocd-applications-parents', environment);
}

function applyResourceManifests(resourceName: ResourceName, environment: Environment) {
    const resourceDir = getResourceAbsolutePath(resourceName, environment);

    const applyManifests = (subDir: string) => {
        const subDirPath = path.join(resourceDir, subDir);
        const manifestsCount = Number(sh.exec(`ls ${subDirPath} | wc -l`).stdout.trim());
        const isEmptyDir = manifestsCount === 0;
        if (isEmptyDir) return;

        sh.exec(`kubectl apply -R -f  ${subDirPath}`);
    };

    const sealedSecrets: ResourceName = 'sealed-secrets';
    [sealedSecrets, '0-crd', '1-manifest'].forEach(applyManifests);
}

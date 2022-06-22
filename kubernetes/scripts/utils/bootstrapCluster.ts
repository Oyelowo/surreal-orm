#!/usr/bin/env ts-node

import path from 'path';
import sh from 'shelljs';
import { namespaceNames } from '../../resources/namespaces/util';
import { helmChartsInfo } from '../../resources/shared/helmChartInfo';
import { getResourceAbsolutePath } from '../../resources/shared/manifestsDirectory';
import { Environment, ResourceName } from '../../resources/types/own-types';
import { setupPlainSecretTSFiles } from '../secretsManagement/setupSecrets';
import { generateAllSealedSecrets } from './generateAllSealedSecrets';
import { generateManifests } from './generateManifests';
import { getImageTagsFromDir } from './getImageTagsFromDir';

export async function bootstrapCluster(environment: Environment) {
    const imageTags = await getImageTagsFromDir();

    await generateManifests({
        environment,
        imageTags,
    });

    setupPlainSecretTSFiles();

    /*
       This requires the above step with initial cluster setup making sure that the sealed secret controller is
       running in the cluster */

    // # Apply namespace first
    applyResourceManifests('namespace-names', environment);

    // # Apply setups with sealed secret controller
    applyResourceManifests('sealed-secrets', environment);

    const sealedSecretsName: ResourceName = 'sealed-secrets';
    // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
    sh.exec(`kubectl rollout status deployment/${sealedSecretsName} -n=${namespaceNames.kubeSystem}`);

    // # Apply setups with cert-manager controller
    applyResourceManifests('cert-manager', environment);

    // # Wait for cert-manager and cert-manager-trust controllers to be in running phase so that we can use it to encrypt secrets
    const { certManager, certManagerTrust } = helmChartsInfo.jetspack.charts;
    sh.exec(`kubectl rollout status deployment/${certManager.chart} -n=${namespaceNames.certManager}`);
    sh.exec(`kubectl rollout status deployment/${certManagerTrust.chart} -n=${namespaceNames.certManager}`);

    // # Apply setups with linkerd controller
    applyResourceManifests('linkerd', environment);
    applyResourceManifests('linkerd-viz', environment);

    await generateAllSealedSecrets({
        environment,
    });

    // TODO: could conditionally check the installation of argocd also cos it may not be necessary for local dev
    applyResourceManifests('argocd', environment);
    // TODO: Split bootstrap process from restart from update
    sh.exec('kubectl -n argocd rollout restart deployment argocd-argo-cd-server');

    // TODO: Only apply this in non prod environment
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

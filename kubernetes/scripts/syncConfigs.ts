import { KubeObject } from './utils/kubeObject/kubeObject.js';
import { syncCrdsCode } from './utils/kubeObject/syncCrdsCode.js';
import { PlainSecretJsonConfig } from './utils/plainSecretJsonConfig.js';
import { syncEtcHostsWithCustomHosts } from './utils/syncEtcHostsWithCustomHosts.js';
import { syncHelmChartTypesDeclarations } from './utils/syncHelmChartTypesDeclarations.js';

async function main() {
    syncEtcHostsWithCustomHosts();
    syncHelmChartTypesDeclarations();

    PlainSecretJsonConfig.syncAll();

    // Use local manifests to syn/generate new CRD codes
    const kubeObjectInstance = new KubeObject('local');
    await kubeObjectInstance.generateManifests();
    const crds = kubeObjectInstance.getOfAKind('CustomResourceDefinition');
    syncCrdsCode(crds);
}

main().catch((e) => `problem syncing configs. Error: ${e}`);

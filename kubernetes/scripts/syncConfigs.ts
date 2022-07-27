import { KubeObject } from './utils/kubeObject/kubeObject';
import { syncCrdsCode } from './utils/kubeObject/syncCrdsCode';
import { PlainSecretJsonConfig } from './utils/plainSecretJsonConfig';
import { syncEtcHostsWithCustomHosts } from './utils/syncEtcHostsWithCustomHosts';
import { syncHelmChartTypesDeclarations } from './utils/syncHelmChartTypesDeclarations';

async function main() {
    syncEtcHostsWithCustomHosts();
    syncHelmChartTypesDeclarations();

    PlainSecretJsonConfig.syncAll();

    
    // Use local manifests to syn/generate new CRD codes
    const kubeObjectInstance = new KubeObject('local')
    await kubeObjectInstance.generateManifests()
    const crds = kubeObjectInstance.getOfAKind('CustomResourceDefinition');
    syncCrdsCode(crds);
}

main().catch(e=> `problem syncing configs. Error: ${e}`)
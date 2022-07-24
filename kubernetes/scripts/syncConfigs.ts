import { KubeObject } from './utils/kubeObject/kubeObject';
import { syncCrdsCode } from './utils/kubeObject/syncCrdsCode';
import { PlainSecretJsonConfig } from './utils/plainSecretJsonConfig';
import { syncEtcHostsWithCustomHosts } from './utils/syncEtcHostsWithCustomHosts';
import { syncHelmChartTypesDeclarations } from './utils/syncHelmChartTypesDeclarations';

syncEtcHostsWithCustomHosts();
syncHelmChartTypesDeclarations();
PlainSecretJsonConfig.syncAll();


// Use local manifests to syn/generate new CRD codes
const crds = new KubeObject("local").getOfAKind('CustomResourceDefinition');
syncCrdsCode(crds);

import { KubeObject } from './utils/kubeObject/kubeObject';
import { syncCrdsCode } from './utils/kubeObject/syncCrdsCode';
import { PlainSecretJsonConfig } from './utils/plainSecretJsonConfig';
import { syncEtcHostsWithCustomHosts } from './utils/syncEtcHostsWithCustomHosts';
import { syncHelmChartTypesDeclarations } from './utils/syncHelmChartTypesDeclarations';

syncEtcHostsWithCustomHosts();
syncHelmChartTypesDeclarations();
PlainSecretJsonConfig.syncAll()

// The enviroment does not matter here
const kubeObject = new KubeObject('local');
syncCrdsCode(kubeObject.getOfAKind('CustomResourceDefinition'));

import { KubeObject } from './utils/kubeObject/kubeObject';
import { syncSecretsTsFiles } from './secretsManagement/syncSecretsTsFiles';
import { syncCrdsCode } from './utils/kubeObject/syncCrdsCode';
import { syncEtcHostsWithCustomHosts } from './utils/syncEtcHostsWithCustomHosts';
import { syncHelmChartTypesDeclarations } from './utils/syncHelmChartTypesDeclarations';

syncEtcHostsWithCustomHosts();
syncHelmChartTypesDeclarations();
syncSecretsTsFiles();

const kubeObject = new KubeObject('local');
syncCrdsCode(kubeObject.getOfAKind('CustomResourceDefinition'));

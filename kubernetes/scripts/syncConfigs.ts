import { setupOrSyncPlainSecretTSFiles } from './secretsManagement/setupSecrets';
import { syncEtcHostsWithCustomHosts } from './utils/syncEtcHostsWithCustomHosts';
import { syncHelmChartTypesDeclarations } from './utils/syncHelmChartTypesDeclarations';

syncEtcHostsWithCustomHosts();
syncHelmChartTypesDeclarations();
setupOrSyncPlainSecretTSFiles();

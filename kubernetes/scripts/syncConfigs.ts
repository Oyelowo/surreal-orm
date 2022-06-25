import { setupOrSyncPlainSecretTSFiles } from './secretsManagement/setupSecrets';
import { syncEtcHostsWithCustomHosts } from './syncEtcHostsWithCustomHosts';
import { syncHelmChartTypesDeclarations } from './utils/syncHelmChartTypesDeclarations';


syncEtcHostsWithCustomHosts();
syncHelmChartTypesDeclarations();
setupOrSyncPlainSecretTSFiles();
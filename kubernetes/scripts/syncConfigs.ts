import { setupOrSyncPlainSecretTSFiles } from './secretsManagement/setupSecrets';
import { syncHelmChartTypesDeclarations } from './utils/syncHelmChartTypesDeclarations';

syncHelmChartTypesDeclarations();
setupOrSyncPlainSecretTSFiles();

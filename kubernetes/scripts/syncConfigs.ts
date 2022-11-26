import { PlainSecretsManager } from "./utils/plainSecretsManager.js";
import { syncCrdsCode } from "./utils/syncCrdsCode.js";
import { syncEtcHostsWithCustomHosts } from "./utils/syncEtcHostsWithCustomHosts.js";
import { syncHelmChartTypesDeclarations } from "./utils/syncHelmChartTypesDeclarations.js";

async function main() {
	syncEtcHostsWithCustomHosts();
	syncHelmChartTypesDeclarations();

	PlainSecretsManager.syncAll();
	syncCrdsCode();
}

await main();

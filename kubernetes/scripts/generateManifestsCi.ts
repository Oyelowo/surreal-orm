import { ARGV_ENVIRONMENTS } from "./utils/argv.js";
import { KubeObject } from "./utils/kubeObject/kubeObject.js";

/* 
Does not handle sealed secret generation/syncing
*/

async function main() {
	const kubeObject = new KubeObject(ARGV_ENVIRONMENTS.environment);
	await kubeObject.generateManifests();
}

await main();

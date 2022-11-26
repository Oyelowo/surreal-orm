import { getMainBaseDir } from "../../../src/shared/directoriesManager.js";
import c from "chalk";
import p from "node:path";
import sh from "shelljs";
import { handleShellError } from "../shared.js";
import { KubeObject } from "./kubeObject.js";
import type { TKubeObject } from "./kubeObject.js";
import path from "node:path";
import { Environment } from "../../../src/types/ownTypes.js";
import {
	EnvironmentVariables,
	imageTags,
} from "../../../src/shared/environmentVariablesForManifests.js";

/*
GENERATE ALL KUBERNETES MANIFESTS USING PULUMI
*/

const mainDir = getMainBaseDir();
export const tsConfigPath = path.join(mainDir, "tsconfig.pulumi.json");
export async function generateManifests(kubeObject: KubeObject) {
	sh.exec("make install");
	const loginDir = path.join(mainDir, ".login");
	sh.rm("-rf", loginDir);
	sh.mkdir("-p", loginDir);

	// https://www.pulumi.com/docs/intro/concepts/state/#logging-into-the-local-filesystem-backend
	sh.exec(`pulumi login file://${loginDir}`);

	sh.echo(c.blueBright("DELETE EXISTING RESOURCES(except sealed secrets)"));

	const removeNonSealedSecrets = (obj: TKubeObject) => {
		const isSealedSecret = obj.kind === "SealedSecret";
		!isSealedSecret && sh.rm("-rf", obj.path);
	};

	kubeObject.getAll().forEach(removeNonSealedSecrets);

	handleShellError(
		sh.rm("-rf", `${p.join(getMainBaseDir(), "Pulumi.dev.yaml")}`),
	);
	handleShellError(
		sh.exec(
			"export PULUMI_CONFIG_PASSPHRASE='not-needed' && pulumi stack init --stack dev",
		),
	);

	handleShellError(
		sh.exec(
			`
        ${getEnvVarsForScript({ environment: kubeObject.getEnvironment() })}
        export PULUMI_CONFIG_PASSPHRASE="not-needed"
        export PULUMI_NODEJS_TRANSPILE_ONLY=true
        export PULUMI_SKIP_CONFIRMATIONS=true
        export PULUMI_NODEJS_TSCONFIG_PATH=${tsConfigPath}
        pulumi up --yes --skip-preview --stack dev
       `,
		),
	);
	sh.rm("-rf", loginDir);
}
// export function getEnvVarsForScript(environment: Environment, imageTags: ImageTags) {
function getEnvVarsForScript({ environment }: { environment: Environment }) {
	// Not really necessary to have the image tags as environment variable
	// here aas I'm already using it directly in the manifests function
	const imageEnvVarSetterForPulumi = Object.entries(imageTags)
		.map(([k, v]) => `export ${k}=${v}`)
		.join(" ");
	return `
      ${imageEnvVarSetterForPulumi} 
      export ${"ENVIRONMENT"}=${environment}  
  `;
}

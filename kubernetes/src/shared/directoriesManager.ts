import * as k8s from "@pulumi/kubernetes";
// import url from 'node:url';
import path from "node:path";
import { v4 as uuid } from "uuid";
import {
	Environment,
	InfrastructureName,
	ServiceName,
	TInfrastructure,
	TServices,
} from "../types/ownTypes.js";

// const __filename = url.fileURLToPath(import.meta.url);
// const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

export const getMainBaseDir = () => {
	// const mainBaseDir = path.join(__dirname, '..', '..');
	// return mainBaseDir
	// This is more resilient to folder structure change compared to
	// the above but requires user to actually run program from the root
	// Keep the alternative for reference purpose.
	return process.cwd();
};

export const getPlainSecretsConfigFilesBaseDir = () => {
	return path.join(getMainBaseDir(), ".secrets");
};

export const getGeneratedCrdsCodeDir = () => {
	const baseDir = getMainBaseDir();
	return path.join(baseDir, "generatedCrdsTs");
};

type ResourcePaths =
	| `${TInfrastructure}/${InfrastructureName}`
	| `${TServices}/${ServiceName}`;
export type ResourceOutputDirProps = {
	outputDirectory: ResourcePaths;
	environment: Environment;
};

/** Directory of a generated manifests for an environment(local/production etc)  */
export const getGeneratedEnvManifestsDir = (environment: Environment) => {
	return path.join(getMainBaseDir(), "generatedManifests", environment);
};

export const getResourceAbsolutePath = (
	props: ResourceOutputDirProps,
): string => {
	return path.join(
		getGeneratedEnvManifestsDir(props.environment),
		path.normalize(props.outputDirectory),
	);
};

export function getResourceRelativePath(props: ResourceOutputDirProps): string {
	const pathAbsolute = getResourceAbsolutePath(props);
	return path.relative(getMainBaseDir(), pathAbsolute);
}

export function getResourceProvider(
	props: ResourceOutputDirProps,
): k8s.Provider {
	return new k8s.Provider(`${props.outputDirectory}-${uuid()}`, {
		renderYamlToDirectory: getResourceAbsolutePath(props),
	});
}

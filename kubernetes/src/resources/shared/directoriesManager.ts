import { ServiceName } from './../types/ownTypes.js';
import * as k8s from '@pulumi/kubernetes';
import url from 'node:url';
import path from 'node:path';
import { v4 as uuid } from 'uuid';
import { Environment, InfrastructureName } from '../types/ownTypes.js';

// const __filename = url.fileURLToPath(import.meta.url);
const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

export const getMainBaseDir = () => {
    const mainBaseDir = path.join(__dirname, '..', '..', '..');
    return mainBaseDir;
};

export const getPlainSecretsConfigFilesBaseDir = () => {
    return path.join(getMainBaseDir(), '.secrets');
};

export const getGeneratedCrdsCodeDir = () => {
    const baseDir = getMainBaseDir();
    return path.join(baseDir, 'generatedCrdsTs');
};

export type ResourceOutputDirProps = {
    outputDirectory: `infrastructure/${InfrastructureName}` | `services/${ServiceName}`;
    environment: Environment;
};

/** Directory of a generated manifests for an environment(local/production etc)  */
export const getGeneratedEnvManifestsDir = (environment: Environment) => {
    return path.join(getMainBaseDir(), 'generatedManifests', environment);
};

export const getResourceAbsolutePath = (props: ResourceOutputDirProps): string => {
    return path.join(getGeneratedEnvManifestsDir(props.environment), path.normalize(props.outputDirectory));
};

export function getResourceRelativePath(props: ResourceOutputDirProps): string {
    const pathAbsolute = getResourceAbsolutePath(props);
    return path.relative(getMainBaseDir(), pathAbsolute);
}

export function getResourceProvider(props: ResourceOutputDirProps): k8s.Provider {
    return new k8s.Provider(`${props.outputDirectory}-${uuid()}`, {
        renderYamlToDirectory: getResourceAbsolutePath(props),
    });
}

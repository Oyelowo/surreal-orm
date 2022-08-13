import { ServiceName } from './../types/ownTypes';
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

export type ResourcePathProps = {
    resourcePath: `infrastructure/${InfrastructureName}` | `services/${ServiceName}`;
    environment: Environment;
};

/** Directory of a generated manifests for an environment(local/production etc)  */
export const getGeneratedEnvManifestsDir = (environment: Environment) => {
    return path.join(getMainBaseDir(), 'generatedManifests', environment);
};

export const getResourceAbsolutePath = (props: ResourcePathProps): string => {
    return path.join(getGeneratedEnvManifestsDir(props.environment), path.normalize(props.resourcePath));
};

export function getResourceRelativePath(props: ResourcePathProps): string {
    const pathAbsolute = getResourceAbsolutePath(props);
    return path.relative(getMainBaseDir(), pathAbsolute);
}

export function getResourceProvider(props: ResourcePathProps): k8s.Provider {
    return new k8s.Provider(`${props.resourcePath}-${uuid()}`, {
        renderYamlToDirectory: getResourceAbsolutePath(props),
    });
}

import * as k8s from '@pulumi/kubernetes';
import url from 'node:url';
import path from 'node:path';
import { v4 as uuid } from 'uuid';
import { Environment, ResourceName, ResourceType } from '../types/ownTypes.js';

// const __filename = url.fileURLToPath(import.meta.url);
const __dirname = url.fileURLToPath(new URL('.', import.meta.url));

export const getMainBaseDir = () => {
    const mainBaseDir = path.join(__dirname, '..', '..', '..');
    return mainBaseDir;
};

export const getManifestsBaseDir = () => {
    const MANIFESTS_DIR = path.join(getMainBaseDir(), 'generatedManifests');
    return MANIFESTS_DIR;
};

export const getPlainSecretsConfigFilesBaseDir = () => {
    return path.join(getMainBaseDir(), '.secrets');
};

export const getGeneratedCrdsCodeDir = () => {
    const baseDir = getMainBaseDir();
    return path.join(baseDir, 'generatedCrdsTs');
};

/**
 * e.g generatedManifests/local/infrastructure/1-manifest
 *                               /infrastructure/1-crd
 *                               /infrastructure/sealed-secrets
 */
export const getRepoPathFromAbsolutePath = (absolutePath: string) => {
    const toolPath = absolutePath.split('/kubernetes/').at(-1);
    if (!toolPath) {
        throw new Error('path not found');
    }
    return path.join('kubernetes', toolPath);
};

// function getResourceProperties<T>(
//     resourceName: ResourceName,
//     onGetResourceProperties: (resourceName: ResourceType) => T
// ): T {
//     switch (resourceName) {
//         case 'react-web':
//         case 'graphql-mongo':
//         case 'graphql-postgres':
//         case 'grpc-mongo': {
//             return onGetResourceProperties('services');
//         }

//         case 'argocd':
//         case 'cert-manager':
//         case 'linkerd':
//         case 'sealed-secrets':
//         case 'linkerd-viz':
//         case 'namespaces':
//         case 'nginx-ingress':
//         case 'argocd-applications-children-infrastructure':
//         case 'argocd-applications-children-services':
//         case 'argocd-applications-parents': {
//             return onGetResourceProperties('infrastructure');
//         }
//     }
//     return assertUnreachable(resourceName);
// }

export function assertUnreachable(_: never): never {
    throw new Error("Didn't expect to get here");
}

export type GetPathToResourceProps = {
    resourceType: ResourceType;
    resourceName: ResourceName;
    environment: Environment;
    /** Note: Optional: This is automatically derived but here specifically for testing.
     * You likely should not need to provide it outside of testing*/
    // envManifestDir?: string;
};

type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;

/** Directory of a generated manifests for an environment(local/production etc)  */
export const getGeneratedEnvManifestsDir = (environment: Environment) => {
    const MANIFESTS_DIR = getManifestsBaseDir();
    return path.join(MANIFESTS_DIR, environment);
};

export const getPathToResourceType = (
    props: Omit<GetPathToResourceProps, 'resourceName'> & { manifestsDir?: string }
): string => {
    const resourcePath = path.join(
        props.manifestsDir ?? getGeneratedEnvManifestsDir(props.environment),
        props.resourceType
    );
    return resourcePath;
};

export const getResourceAbsolutePath = (props: GetPathToResourceProps & { manifestsDir?: string }): string => {
    return path.join(getPathToResourceType(props), props.resourceName);
};

// export const getResourceAbsolutePathForTest = (props: GetPathToResourceProps & { manifestsDir: string }): string => {
//     return path.join(props.manifestsDir, props.resourceType, props.resourceName);
// };

export function getResourceRelativePath(props: GetPathToResourceProps): string {
    const pathAbsolute = getResourceAbsolutePath(props);
    return getRepoPathFromAbsolutePath(pathAbsolute);
}

export function getResourceProvider(props: GetPathToResourceProps): k8s.Provider {
    return new k8s.Provider(`${props.resourceType}-${props.resourceName}-${uuid()}`, {
        renderYamlToDirectory: getResourceAbsolutePath(props),
    });
}

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

/** Directory of a generated manifests for an environment(local/production etc)  */
export const getGeneratedEnvManifestsDir = (environment: Environment) => {
    const MANIFESTS_DIR = getManifestsBaseDir();
    return path.join(MANIFESTS_DIR, environment);
};

export const getGeneratedCrdsCodeDir = () => {
    const baseDir = getMainBaseDir();
    return path.join(baseDir, 'generatedCrdsTs');
};

export const getPathToResourcesDir = (
    resourceName: ResourceName,
    resourceType: ResourceType,
    environment: Environment
) => {
    return path.join(getGeneratedEnvManifestsDir(environment), resourceType, resourceName);
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

function getResourceProperties<T>(
    resourceName: ResourceName,
    onGetResourceProperties: (resourceName: ResourceType) => T
): T {
    switch (resourceName) {
        case 'react-web':
        case 'graphql-mongo':
        case 'graphql-postgres':
        case 'grpc-mongo': {
            return onGetResourceProperties('services');
        }

        case 'argocd':
        case 'cert-manager':
        case 'linkerd':
        case 'sealed-secrets':
        case 'linkerd-viz':
        case 'namespaces':
        case 'nginx-ingress':
        case 'argocd-applications-children-infrastructure':
        case 'argocd-applications-children-services':
        case 'argocd-applications-parents': {
            return onGetResourceProperties('infrastructure');
        }
    }
    return assertUnreachable(resourceName);
}

export function assertUnreachable(_: never): never {
    throw new Error("Didn't expect to get here");
}

type GetPathToResourceProps = {
    resourceType: ResourceType;
    resourceName: ResourceName;
    environment: Environment;
};

type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;

export const getPathToResourceType = (props: Omit<GetPathToResourceProps, 'resourceName'>): string => {
    const resourcePath = path.join(getGeneratedEnvManifestsDir(props.environment), props.resourceType);
    return resourcePath;
};

export const getPathToResource = (props: GetPathToResourceProps): string => {
    return path.join(getPathToResourceType(props), props.resourceName);
};

export function getResourceAbsolutePath(resourceName: ResourceName, environment: Environment): string {
    return getResourceProperties(resourceName, (resourceType) => {
        return getPathToResource({
            resourceName,
            resourceType,
            environment,
        });
    });
}

export function getResourceRelativePath(resourceName: ResourceName, environment: Environment): string {
    const pathAbsolute = getResourceAbsolutePath(resourceName, environment);
    return getRepoPathFromAbsolutePath(pathAbsolute);
}

export function getResourceProvider(resourceName: ResourceName, environment: Environment): k8s.Provider {
    return getResourceProperties(resourceName, (resourceType) => {
        return new k8s.Provider(`${resourceType}-${resourceName}-${uuid()}`, {
            renderYamlToDirectory: getResourceAbsolutePath(resourceName, environment),
        });
    });
}

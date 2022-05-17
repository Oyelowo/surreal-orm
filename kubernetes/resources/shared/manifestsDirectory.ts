import { ServiceName, Environment, ResourceName, ResourceType } from './types/own-types'
import path from 'path'
import sh from 'shelljs'
import * as k8s from '@pulumi/kubernetes'
import { v4 as uuid } from 'uuid'

// TODO:  Unify all the resourceType/resourceName path utils into a singular function e.g
// 12th May, 2022: Get base dir for all kubernetes code: For now from the repo/monorepo root, it is: ./kubernetes
export const getMainBaseDir = () => {
    const mainBaseDir = path.join(__dirname, '..', '..')
    return mainBaseDir
}

export const getManifestsBaseDir = () => {
    const MANIFESTS_DIR = path.join(getMainBaseDir(), 'manifests')
    return MANIFESTS_DIR
}

export const getPlainSecretsConfigFilesBaseDir = () => {
    return path.join(getMainBaseDir(), '.secrets')
}

export const getGeneratedEnvManifestsDir = (environment: Environment) => {
    const MANIFESTS_DIR = getManifestsBaseDir()
    return path.join(MANIFESTS_DIR, 'generated', environment)
}

export const getPathToResourcesDir = (
    resourceName: ResourceName,
    resourceType: ResourceType,
    environment: Environment
) => {
    return path.join(getGeneratedEnvManifestsDir(environment), resourceType, resourceName)
    // return `kubernetes/manifests/generated/${environment}/services/${appName}`;
}

/**
 * e.g /manifests/generated/local/infrastructure/1-manifests
 *                               /infrastructure/1-crd
 *                               /infrastructure/sealed-secrets
 */

export const getRepoPathFromAbsolutePath = (absolutePath: string) => {
    const toolPath = absolutePath.split('/kubernetes/').at(-1)
    if (!toolPath) {
        throw new Error(`path not found`)
    }
    return path.join('kubernetes', toolPath)
}

function getResourceProperties<T>(
    resourceName: ResourceName,
    onGetResourceProperties: (resourceName: ResourceType) => T
): T {
    switch (resourceName) {
        case 'react-web':
        case 'graphql-mongo':
        case 'graphql-postgres':
        case 'grpc-mongo': {
            return onGetResourceProperties('services')
        }

        case 'argocd':
        case 'cert-manager':
        case 'linkerd':
        case 'sealed-secrets':
        case 'linkerd-viz':
        case 'namespace-names':
        case 'nginx-ingress':
        case 'argocd-applications-children-infrastructure':
        case 'argocd-applications-children-services':
        case 'argocd-applications-parents': {
            return onGetResourceProperties('infrastructure')
        }
    }
    return assertUnreachable(resourceName)
}

export function assertUnreachable(x: never): never {
    throw new Error("Didn't expect to get here")
}

type GetPathToResourceProps = {
    resourceType: ResourceType
    // resourceName: Omit<ArgoApplicationName | AppName, "service">
    resourceName: ResourceName
    environment: Environment
}

type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>

export const getPathToResourceType = (props: Omit<GetPathToResourceProps, 'resourceName'>): string => {
    const resourcePath = path.join(getGeneratedEnvManifestsDir(props.environment), props.resourceType)
    return resourcePath
}

export const getPathToResource = (props: GetPathToResourceProps): string => {
    return path.join(getPathToResourceType(props), props.resourceName)
}

export function getResourceAbsolutePath(resourceName: ResourceName, environment: Environment): string {
    return getResourceProperties(resourceName, (resourceType) => {
        return getPathToResource({
            resourceName,
            resourceType,
            environment,
        })
    })
}

export function getResourceRelativePath(resourceName: ResourceName, environment: Environment): string {
    const pathAbsolute = getResourceAbsolutePath(resourceName, environment)
    return getRepoPathFromAbsolutePath(pathAbsolute)
}

export function getResourceProvider(resourceName: ResourceName, environment: Environment): k8s.Provider {
    return getResourceProperties(resourceName, (resourceType) => {
        return new k8s.Provider(`${resourceType}-${resourceName}-${uuid()}`, {
            renderYamlToDirectory: getResourceAbsolutePath(resourceName, environment),
        })
    })
}

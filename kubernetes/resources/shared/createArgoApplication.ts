import * as kx from '@pulumi/kubernetesx'
import { Resource } from '@pulumi/pulumi'
import * as argocd from '../../crd2pulumi/argocd'
import { getSecretsForResource } from '../../scripts/secretsManagement/getSecretsForApp'
import { NamespaceName, namespaceNames } from './../namespaces/util'
import { APPLICATION_AUTOMERGE_ANNOTATION } from './constants'
import { getResourceProvider, getResourceRelativePath } from './manifestsDirectory'
import { ArgocdAppResourceName, ResourceName } from './types/own-types'
import { getEnvironmentVariables } from './validations'

const { ENVIRONMENT } = getEnvironmentVariables()

type Metadata = {}

type ArgocdApplicationProps = {
    namespace: NamespaceName
    sourceResourceName: ArgocdAppResourceName
    resourceName: ResourceName
    parent?: Resource
}

// TODO: Add jsdoc to describe the parameters
export function createArgocdApplication({
    resourceName,
    sourceResourceName,
    namespace,
    parent,
}: ArgocdApplicationProps) {
    const argocdApplication = new argocd.argoproj.v1alpha1.Application(
        resourceName,
        {
            metadata: {
                name: resourceName,
                namespace: namespaceNames.argocd,
                annotations: {
                    finalizers: ['resources-finalizer.argocd.argoproj.io'] as any,
                    // Maybe use? argocd.argoproj.io / hook: PreSync
                },
            },
            spec: {
                project: 'default',
                destination: {
                    server: 'https://kubernetes.default.svc',
                    namespace,
                },
                source: {
                    repoURL: 'https://github.com/Oyelowo/modern-distributed-app-template',
                    path: getResourceRelativePath(sourceResourceName, ENVIRONMENT),
                    targetRevision: 'HEAD',
                    directory: {
                        recurse: true,
                    },
                },
                // syncPolicy: {
                //   automated: {
                //     prune: true,
                //     selfHeal: true,
                //   },
                // },
            },
        },
        {
            provider: getResourceProvider(resourceName, ENVIRONMENT),
            parent,
        }
    )

    return argocdApplication
}

const metadata: Omit<Metadata, 'argoApplicationName' | 'resourceType'> = {
    name: 'argocd-applications-secret',
    namespace: namespaceNames.argocd,
    labels: {
        'argocd.argoproj.io/secret-type': 'repository',
    },
}

const secrets = getSecretsForResource('argocd', ENVIRONMENT)
export const argoCDApplicationsSecret = new kx.Secret(
    `argocd-secret`,
    {
        stringData: {
            ...secrets,
        },
        metadata: {
            ...metadata,
            annotations: {
                ...APPLICATION_AUTOMERGE_ANNOTATION,
            },
        },
    },
    { provider: getResourceProvider('argocd-applications-parents', ENVIRONMENT) }
)

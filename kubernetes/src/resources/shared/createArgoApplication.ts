import * as kx from '@pulumi/kubernetesx';
import { Resource } from '@pulumi/pulumi';
import crds from '../../../generatedCrdsTs/index.js';
import { Namespace, namespaces } from '../infrastructure/namespaces/util.js';
import { GetPathToResourceProps, getResourceProvider, getResourceRelativePath } from './directoriesManager.js';
import { ResourceName } from '../types/ownTypes.js';
import { getEnvironmentVariables } from './validations.js';
import { PlainSecretJsonConfig } from '../../../scripts/utils/plainSecretJsonConfig.js';

type ArgocdApplicationProps = Omit<GetPathToResourceProps, 'resourceName'> & {
    namespace: Namespace;
    outputSubDirName: ResourceName;
    /** Source is a reference to the location of the application's manifests we are generating app for. */
    sourceApplication: ResourceName;
    // Typically services/infrastructure under which specific app is nested
    parent?: Resource;
};

export function createArgocdApplication({
    sourceApplication,
    outputSubDirName,
    environment,
    resourceType,
    namespace,
    parent,
}: ArgocdApplicationProps) {
    const argocdApplication = new crds.argoproj.v1alpha1.Application(
        sourceApplication,
        {
            metadata: {
                name: sourceApplication,
                namespace: namespaces.argocd,
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
                    path: getResourceRelativePath({
                        resourceName: sourceApplication,
                        environment,
                        resourceType,
                    }),
                    targetRevision: 'HEAD',
                    directory: {
                        recurse: true,
                    },
                },
                syncPolicy: {
                    automated: {
                        prune: true,
                        selfHeal: true,
                    },
                },
            },
        },
        {
            provider: getResourceProvider({ resourceName: outputSubDirName, environment, resourceType }),
            parent,
        }
    );

    return argocdApplication;
}

const metadata = {
    name: 'argocd-applications-secret',
    namespace: namespaces.argocd,
    labels: {
        'argocd.argoproj.io/secret-type': 'repository',
    },
};

const { ENVIRONMENT } = getEnvironmentVariables();
const secrets = new PlainSecretJsonConfig('argocd', ENVIRONMENT).getSecrets();
export const argoCDApplicationsSecret = new kx.Secret(
    'argocd-secret',
    {
        data: {
            ...secrets,
        },
        metadata: {
            ...metadata,
            annotations: {},
        },
    },
    {
        provider: getResourceProvider({
            resourceType: 'infrastructure',
            resourceName: 'argocd-applications-parents',
            environment: ENVIRONMENT,
        }),
    }
);

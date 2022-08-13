import { Environment } from './../types/ownTypes';
import * as kx from '@pulumi/kubernetesx';
import { Resource } from '@pulumi/pulumi';
import crds from '../../../generatedCrdsTs/index.js';
import { Namespace, namespaces } from '../infrastructure/namespaces/util.js';
import { ResourcePathProps, getResourceProvider, getResourceRelativePath } from './directoriesManager.js';
import { ResourceName } from '../types/ownTypes.js';
import { getEnvironmentVariables } from './validations.js';
import { PlainSecretJsonConfig } from '../../../scripts/utils/plainSecretJsonConfig.js';

type ArgocdApplicationProps = {
    namespace: Namespace;
    environment: Environment;
    sourceApplicationName: ResourceName;
    outputPath: ResourcePathProps['resourcePath'];
    // outputSubDirName: ResourceName;
    /** Source is a reference to the location of the application's manifests we are generating app for. */
    sourceApplicationPath: ResourcePathProps['resourcePath'];
    // Typically services/infrastructure under which specific app is nested
    parent?: Resource;
};

export function createArgocdApplication({
    sourceApplicationName,
    sourceApplicationPath,
    outputPath,
    namespace,
    environment,
    parent,
}: ArgocdApplicationProps) {
    const argocdApplication = new crds.argoproj.v1alpha1.Application(
        sourceApplicationName,
        {
            metadata: {
                name: sourceApplicationName,
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
                        resourcePath: sourceApplicationPath,
                        environment,
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
            provider: getResourceProvider({
                resourcePath: outputPath,
                environment,
            }),
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
            resourcePath: `infrastructure/argocd-applications-parents`,
            environment: ENVIRONMENT,
        }),
    }
);

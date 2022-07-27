import * as kx from '@pulumi/kubernetesx';
import { Resource } from '@pulumi/pulumi';
import * as argocd from '../../generatedCrdsTs/argoproj';
import { Namespace, namespaces } from './../infrastructure/namespaces/util';
import { getResourceProvider, getResourceRelativePath } from './manifestsDirectory';
import { ResourceName } from '../types/own-types';
import { getEnvironmentVariables } from './validations';
import { PlainSecretJsonConfig } from '../../scripts/utils/plainSecretJsonConfig';

const { ENVIRONMENT } = getEnvironmentVariables();

type ArgocdApplicationProps = {
    namespace: Namespace;
    outputSubDirName: ResourceName;
    // The application we are trying to generate argo app for.
    sourceApplication: ResourceName;
    // Typically services/infrastructure under which specific app is nested
    parent?: Resource;
};

export function createArgocdApplication({
    sourceApplication,
    outputSubDirName,
    namespace,
    parent,
}: ArgocdApplicationProps) {
    const argocdApplication = new argocd.v1alpha1.Application(
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
                    path: getResourceRelativePath(sourceApplication, ENVIRONMENT),
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
            provider: getResourceProvider(outputSubDirName, ENVIRONMENT),
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

const secrets = new PlainSecretJsonConfig('argocd', ENVIRONMENT).getSecrets();
export const argoCDApplicationsSecret = new kx.Secret(
    'argocd-secret',
    {
        stringData: {
            ...secrets,
        },
        metadata: {
            ...metadata,
            annotations: {},
        },
    },
    { provider: getResourceProvider('argocd-applications-parents', ENVIRONMENT) }
);

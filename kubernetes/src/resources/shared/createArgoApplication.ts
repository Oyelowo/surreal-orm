import { Environment, ResourceNameSchema } from './../types/ownTypes.js';
import * as kx from '@pulumi/kubernetesx';
import { Resource } from '@pulumi/pulumi';
import crds from '../../../generatedCrdsTs/index.js';
import { Namespace, namespaces } from '../infrastructure/namespaces/util.js';
import { ResourceOutputDirProps, getResourceProvider, getResourceRelativePath } from './directoriesManager.js';
import { getEnvironmentVariables } from './validations.js';
import { PlainSecretJsonConfig } from '../../../scripts/utils/plainSecretJsonConfig.js';

type ArgocdApplicationProps = {
    namespace: Namespace;
    environment: Environment;
    /** Where the argocd aaplicaiton itself is going to be generated to. */
    outputDirectory: ResourceOutputDirProps['outputDirectory'];
    /** Source is a reference to the location of the application's manifests we are generating app for. */
    sourceAppDirectory: ResourceOutputDirProps['outputDirectory'];
    // Typically services/infrastructure under which specific app is nested
    parent?: Resource;
};

export function createArgocdApplication({
    sourceAppDirectory,
    outputDirectory,
    namespace,
    environment,
    parent,
}: ArgocdApplicationProps) {
    const sourceApplicationName = ResourceNameSchema.parse(sourceAppDirectory.split('/').at(-1));
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
                        outputDirectory: sourceAppDirectory,
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
                outputDirectory: outputDirectory,
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

const { ENVIRONMENT } = getEnvVarsForKubeManifestGenerator();

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
            outputDirectory: `infrastructure/argocd-applications-parents`,
            environment: ENVIRONMENT,
        }),
    }
);

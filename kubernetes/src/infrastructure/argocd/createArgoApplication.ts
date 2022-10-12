import { Environment, ResourceNameSchema } from '../../types/ownTypes.js';
import * as kx from '@pulumi/kubernetesx';
import { Resource } from '@pulumi/pulumi';
import crds from '../../../generatedCrdsTs/index.js';
import { Namespace, namespaces } from '../namespaces/util.js';
import {
    ResourceOutputDirProps,
    getResourceProvider,
    getResourceRelativePath,
} from '../../shared/directoriesManager.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';
import { PlainSecretsManager } from '../../../scripts/utils/plainSecretsManager.js';
import { toBase64 } from '../../shared/helpers.js';
import _ from 'lodash';

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

const env = getEnvVarsForKubeManifests();
const secrets = new PlainSecretsManager('infrastructure', 'argocd', env.ENVIRONMENT).getSecrets();

const secretsData = {
    ADMIN_PASSWORD: secrets.ADMIN_PASSWORD,
    type: 'git',
    url: secrets.url,
    // url: 'https://github.com/Oyelowo/modern-distributed-app-template',
    username: secrets.GITHUB_USERNAME,
    password: secrets.GITHUB_PASSWORD,
};
const encodedSecretsData = _.mapValues(secretsData, toBase64);

export const argoCDApplicationsSecret = new kx.Secret(
    'argocd-secret',
    {
        data: encodedSecretsData,
        metadata: {
            ...metadata,
            annotations: {},
        },
    },
    {
        provider: getResourceProvider({
            outputDirectory: `infrastructure/argocd-applications-parents`,
            environment: env.ENVIRONMENT,
        }),
    }
);

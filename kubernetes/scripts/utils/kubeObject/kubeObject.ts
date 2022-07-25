import { ResourceName } from './../../../resources/types/own-types';
import { NamespaceList } from '@pulumi/kubernetes/core/v1';
import { Namespace } from './../../../resources/infrastructure/namespaces/util';
import { mergeUnsealedSecretToSealedSecret } from './SealedSecretsManager';
import sh from 'shelljs';
import _ from 'lodash';
import z from 'zod';
import { namespaceSchema } from '../../../resources/infrastructure/namespaces/util';
import { getGeneratedEnvManifestsDir, getResourceAbsolutePath } from '../../../resources/shared/manifestsDirectory';
import type { Environment } from '../../../resources/types/own-types';
import { handleShellError } from '../shared';
import { selectSecretKubeObjectsFromPrompt } from './SecretsSelectorPrompter';
import { generateManifests } from './generateManifests';
import { syncCrdsCode } from './syncCrdsCode';

type ResourceKind =
    | 'Secret'
    | 'Deployment'
    | 'Service'
    | 'Configmap'
    | 'Pod'
    | 'SealedSecret'
    | 'CustomResourceDefinition';




const kubeObjectSchema = z.object({
    kind: z.string(),
    apiVersion: z.string(),
    type: z.string().optional(),
    path: z.string(),
    metadata: z.object({
        name: z.string(),
        // CRDS have namespace as null
        namespace: namespaceSchema.optional(),
        annotations: z.record(z.string()),
    }),
    spec: z
        .object({
            encryptedData: z.record(z.string().nullable()).optional(), // For sealed secrets
            // CRDS have namespace as null
            template: z.any().optional(), //Dont care about this yet
        })
        .optional(),
    data: z.record(z.string().nullable()).optional(),
    stringData: z.record(z.string().nullable()).optional(),
});




type KubeObjectSchema = Required<z.infer<typeof kubeObjectSchema>>;

type CreateKubeObject<K extends ResourceKind> = KubeObjectSchema & {
    kind: Extract<ResourceKind, K>;
};


export type TSecretKubeObject = CreateKubeObject<'Secret'> & {
    selectedSecretsForUpdate?: string[] | null;
};
export type TSealedSecretKubeObject = CreateKubeObject<'SealedSecret'>;
export type TCustomResourceDefinitionObject = CreateKubeObject<'CustomResourceDefinition'>;
export type TKubeObject = TSecretKubeObject | TSealedSecretKubeObject | TCustomResourceDefinitionObject;

export class KubeObject {
    #kubeObjectsAll: TKubeObject[];

    constructor(private environment: Environment) {
        this.#kubeObjectsAll = this.syncAll().getAll();
    }

    getEnvironment = () => this.environment;

    getForApp = (resourceName: ResourceName): TKubeObject[] => {
        const envDir = getResourceAbsolutePath(resourceName, this.environment);
        return this.#kubeObjectsAll.filter((m) => {
            const manifestIsWithinDir = (demarcator: '/' | '\\') => m.path.startsWith(`${envDir}${demarcator}`);
            return manifestIsWithinDir('/') || manifestIsWithinDir('\\');
        });
    };

    getAll = (): TKubeObject[] => {
        return this.#kubeObjectsAll;
    };

    generateManifests = async () => {
        await generateManifests(this);
        syncCrdsCode(this.getOfAKind('CustomResourceDefinition'));
        this.syncAll();
    };

    /** Extract information from all the manifests for an environment(local, staging etc)  */
    syncAll = () => {
        const envDir = getGeneratedEnvManifestsDir(this.environment);
        const manifestsPaths = this.#getManifestsPathWithinDir(envDir);
        const exec = (cmd: string) => handleShellError(sh.exec(cmd, { silent: true })).stdout;

        this.#kubeObjectsAll = manifestsPaths.reduce<TKubeObject[]>((acc, path, i) => {
            if (!path) return acc;
            console.log('Extracting info from manifest', i);

            const info = JSON.parse(exec(`cat ${path.trim()} | yq '.' -o json`));

            if (_.isEmpty(info)) return acc;
            // let's mutate to make it a bit faster and should be okay since we only do it here
            info.path = path;

            const updatedPath = kubeObjectSchema.parse(info) as TKubeObject;

            acc.push(updatedPath);
            return acc;
        }, []);
        return this;
    };

    /** Gets all the yaml manifests for an environment(local, staging etc)  */
    #getManifestsPathWithinDir = (environmentManifestsDir: string): string[] => {
        const manifestMatcher = '*ml';
        const allManifests = sh
            .exec(`find ${environmentManifestsDir} -name "${manifestMatcher}"`, {
                silent: true,
            })
            .stdout.trim()
            .split('\n')
            .map((p) => p.trim());
        return allManifests;
    };

    getOfAKind = <K extends ResourceKind>(kind: K): CreateKubeObject<K>[] => {
        return (this.#kubeObjectsAll as CreateKubeObject<K>[])
            .filter((o) => o.kind === kind)
    };



    /**
Sync all Sealed secrets. This is usually useful when you're bootstrapping
a cluster and you typically want to sync/build all sealed secrets from kubernetes
secret objects. @see NOTE: This shoould only be done after sealed secrets controller 
is running because that is required to seal the plain secrets. You can see where this is used.
Side note: In the future, we can also allow this to use public key of the sealed secret controller
which is cached locally but that would be more involved.
*/
    syncSealedSecrets = async () => {
        const secrets: TSecretKubeObject[] = this.getOfAKind('Secret').map((p) => ({
            ...p,
            // Syncs all secrets
            selectedSecretsForUpdate: Object.keys(p.data ?? p.stringData ?? {}),
        }));
        mergeUnsealedSecretToSealedSecret({
            sealedSecretKubeObjects: this.getOfAKind('SealedSecret'),
            secretKubeObjects: secrets,
        });

        // Sync kube object info after sealed secrets manifests have been updated
        this.syncAll();
    };

    /**
Sync only Sealed secrets that are selected from user in  the CLI terminal prompt. This is usually useful 
when you want to update Secrets in an existing cluster. Plain kubernetes
secrets should never be pushed to git but they help to generate sealed secrets.
*/
    syncSealedSecretsWithPrompt = async () => {
        const selectedSecretObjects = await selectSecretKubeObjectsFromPrompt(this.getOfAKind("Secret"));

        mergeUnsealedSecretToSealedSecret({
            sealedSecretKubeObjects: this.getOfAKind('SealedSecret'),
            // Syncs only selected secrets from the CLI prompt
            secretKubeObjects: selectedSecretObjects,
        });

        // Sync kube object info after sealed secrets manifests have been updated
        this.syncAll();
    };
}

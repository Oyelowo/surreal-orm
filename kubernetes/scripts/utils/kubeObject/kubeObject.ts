import { mergeUnsealedSecretToSealedSecret } from './SealedSecretsManager';
import p from 'path';
import sh from 'shelljs';
import _ from 'lodash';
import { z } from 'zod';
import { namespaceSchema } from '../../../resources/infrastructure/namespaces/util';
import { getGeneratedEnvManifestsDir, getResourceAbsolutePath } from '../../../resources/shared/manifestsDirectory';
import { ResourceName, Environment } from '../../../resources/types/own-types';
import { handleShellError } from '../shared';
import { SealedSecretTemplate } from '../../../resources/types/sealedSecretTemplate';
import { selectSecretKubeObjectsFromPrompt } from './SecretsSelectorPrompter';
import { generateManifests } from './generateManifests';
import { getImageTagsFromDir } from '../getImageTagsFromDir';
import { syncCrdsCode } from './syncCrdsCode';

type ResourceKind =
    | 'Secret'
    | 'Deployment'
    | 'Service'
    | 'Configmap'
    | 'Pod'
    | 'SealedSecret'
    | 'CustomResourceDefinition';

const kubernetesResourceInfo = z.object({
    kind: z.string(),
    apiVersion: z.string(),
    type: z.string().optional(),
    path: z.string(),
    metadata: z.object({
        name: z.string(),
        // CRDS have namespace as null
        namespace: namespaceSchema.optional(),
        annotations: z.record(z.string()).transform((p) => p),
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

type kubernetesResourceInfoZod = z.infer<typeof kubernetesResourceInfo>;

// export interface KubeObjectInfo extends kubernetesResourceInfoZod {
//     // We override the object kind type since it's a nonexhasutive list
//     // We also want to allow allow other string types here
//     kind: ResourceKind;
//     // kind: ResourceKind | (string & {});
// }
type CreateKubeObject<K extends ResourceKind> = kubernetesResourceInfoZod & {
    kind: Extract<ResourceKind, K>;
};
export type TSecretKubeObject = CreateKubeObject<'Secret'>;
export type TSealedSecretKubeObject = CreateKubeObject<'SealedSecret'>;
export type TCustomResourceDefinitionObject = CreateKubeObject<'CustomResourceDefinition'>;
export type TKubeObject = TSecretKubeObject | TSealedSecretKubeObject | TCustomResourceDefinitionObject;

export class KubeObject {
    #kubeObjectsAll: TKubeObject[];

    constructor(private environment: Environment) {
        this.#kubeObjectsAll = this.sync().getAll();
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
        this.sync();
    };

    /** Extract information from all the manifests for an environment(local, staging etc)  */
    sync = () => {
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
            const updatedPath = kubernetesResourceInfo.parse(info) as TKubeObject;

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
        return (this.#kubeObjectsAll as CreateKubeObject<K>[]).filter((o) => o.kind === kind);
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
        mergeUnsealedSecretToSealedSecret({
            sealedSecretKubeObjects: this.getOfAKind('SealedSecret'),
            // Syncs all secrets
            secretKubeObjects: this.getOfAKind("Secret"),
        });

        // Sync kube object info after sealed secrets manifests have been updated
        this.sync();
    };

    /**
Sync only Sealed secrets that are selected from user in  the CLI terminal prompt. This is usually useful 
when you want to update Secrets in an existing cluster. Plain kubernetes
secrets should never be pushed to git but they help to generate sealed secrets.
*/
    syncSealedSecretsWithPrompt = async () => {
        const selectedSecretObjects = await selectSecretKubeObjectsFromPrompt(this.getOfAKind('Secret'));

        mergeUnsealedSecretToSealedSecret({
            sealedSecretKubeObjects: this.getOfAKind('SealedSecret'),
            // Syncs only selected secrets from the CLI prompt
            secretKubeObjects: selectedSecretObjects,
        });

        // Sync kube object info after sealed secrets manifests have been updated
        this.sync();
    };
}

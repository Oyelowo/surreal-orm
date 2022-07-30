import { ResourceName } from './../../../resources/types/own-types';
import { mergeUnsealedSecretToSealedSecret } from './SealedSecretsManager';
import sh from 'shelljs';
import _ from 'lodash';
import z from 'zod';
import ramda from "ramda";
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
    syncAll = (): this => {
        const envDir = getGeneratedEnvManifestsDir(this.environment);
        const manifestsPaths = this.#getManifestsPathWithinDir(envDir);
        const exec = (cmd: string) => handleShellError(sh.exec(cmd, { silent: true })).stdout;

        this.#kubeObjectsAll = manifestsPaths.reduce<TKubeObject[]>((acc, path, i) => {
            if (!path) return acc;
            console.log('Extracting info from manifest', i);

            const kubeObject = JSON.parse(exec(`cat ${path.trim()} | yq '.' -o json`));

            if (_.isEmpty(kubeObject)) return acc;
            // let's mutate to make it a bit faster and should be okay since we only do it here
            kubeObject.path = path;

            // Encode stringData into Data field for Secret Objects. This is to
            // ensure consistency and a single source of truth in handling the data.
            if (kubeObject.kind === "Secret") {
                const encodedStringData = _.mapValues(kubeObject.stringData, v => {
                    return Buffer.from(String(v)).toString("base64");
                });
                kubeObject.data = ramda.mergeDeepRight(kubeObject.data ?? {}, encodedStringData);
            }

            const updatedPath = kubeObjectSchema.parse(kubeObject) as TKubeObject;

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
        // console.log(`this.#kubeObjectsAll`, this.#kubeObjectsAll[0])
        // console.log(`(this.#kubeObjectsAll as CreateKubeObject<K>[]).filter((o) => o.kind === kind)`, (this.#kubeObjectsAll as CreateKubeObject<K>[]).filter((o) => o.kind === kind))
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
    syncSealedSecrets = (): void => {
        const secrets: TSecretKubeObject[] = this.getOfAKind('Secret').map((s) => {
            return {
                ...s,
                // Syncs all secrets
                selectedSecretsForUpdate: Object.keys(s?.data ?? {}),
            }
        }) as TSecretKubeObject[];
        // console.log(chalk.cyanBright(`XXXXXXX...Secretssss`, JSON.stringify(secrets, null, 4)))
        // return
        // console.log(chalk.cyanBright(`this.getOfAKind('SealedSecret'). ${this.getOfAKind('SealedSecret')}`))
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
        const selectedSecretObjects = await selectSecretKubeObjectsFromPrompt(this.getOfAKind('Secret'));

        mergeUnsealedSecretToSealedSecret({
            sealedSecretKubeObjects: this.getOfAKind('SealedSecret'),
            // Syncs only selected secrets from the CLI prompt
            secretKubeObjects: selectedSecretObjects,
        });

        // Sync kube object info after sealed secrets manifests have been updated
        this.syncAll();
    };
}

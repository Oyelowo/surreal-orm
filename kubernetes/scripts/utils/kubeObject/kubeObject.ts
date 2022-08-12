import { ResourceName } from '../../../src/resources/types/ownTypes.js';
import { mergeUnsealedSecretToSealedSecret } from './sealedSecretsManager.js';
import { selectSecretKubeObjectsFromPrompt } from './secretsSelectorPrompter.js';
import sh from 'shelljs';
import * as ramda from 'ramda';
import _ from 'lodash';
import z from 'zod';
import { namespaceSchema } from '../../../src/resources/infrastructure/namespaces/util.js';
import {
    getGeneratedEnvManifestsDir,
    getResourceAbsolutePath,
} from '../../../src/resources/shared/directoriesManager.js';
import type { Environment } from '../../../src/resources/types/ownTypes.js';
import { handleShellError } from '../shared.js';
import { generateManifests } from './generateManifests.js';
import { syncCrdsCode } from './syncCrdsCode.js';

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
        annotations: z.record(z.string()).optional(),
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

export type TKubeObjectBaseCommonProps<K extends ResourceKind> = KubeObjectSchema & {
    kind: Extract<ResourceKind, K>;
};

// The following is done to narrow down the type for specific object
// in case it has extra properties and the expose a single `TKubeObject` with
// Resource Kind parameter
// If you want more resources with specific properties, then you
// have to add as above
type KubeObjectCustom =
    | (TKubeObjectBaseCommonProps<'Secret'> & {
          selectedSecretsForUpdate?: string[] | null;
      })
    | TKubeObjectBaseCommonProps<'SealedSecret'>
    | TKubeObjectBaseCommonProps<'CustomResourceDefinition'>
    | TKubeObjectBaseCommonProps<'Deployment'>
    | TKubeObjectBaseCommonProps<'Configmap'>
    | TKubeObjectBaseCommonProps<'Service'>
    | TKubeObjectBaseCommonProps<'Pod'>;

// This helps to narrow the kubernetes object type in case
// it has extra specifc properties
export type TKubeObject<K extends ResourceKind=ResourceKind> = Extract<KubeObjectCustom, { kind: K }>;

export type TSecretKubeObject = TKubeObjectBaseCommonProps<'Secret'> & {
    selectedSecretsForUpdate?: string[] | null;
};
// Add new resource type here like the above if you need more/new specific/narrow type as aboeve.
// The below is the combination of all
export type TKubeObjectAll = TKubeObjectBaseCommonProps<ResourceKind>;

export class KubeObject {
    private kubeObjectsAll: TKubeObjectAll[] = [];

    constructor(private environment: Environment) {
        this.kubeObjectsAll = this.syncAll().getAll();
    }

    getEnvironment = () => this.environment;

    getForApp = (resourceName: ResourceName): TKubeObjectAll[] => {
        const envDir = getResourceAbsolutePath(resourceName, this.environment);
        return this.kubeObjectsAll.filter((m) => {
            const manifestIsWithinDir = (demarcator: '/' | '\\') => m.path.startsWith(`${envDir}${demarcator}`);
            return manifestIsWithinDir('/') || manifestIsWithinDir('\\');
        });
    };

    getAll = (): TKubeObjectAll[] => {
        return this.kubeObjectsAll;
    };

    generateManifests = async () => {
        await generateManifests(this);
        this.syncAll();
        syncCrdsCode(this.getOfAKind('CustomResourceDefinition'));
    };

    getManifestsDir() {
        return getGeneratedEnvManifestsDir(this.environment);
    }
    // #getManifestsDir = () => getGeneratedEnvManifestsDir(this.environment);

    /** Extract information from all the manifests for an environment(local, staging etc)  */
    private syncAll = (): this => {
        const envDir = this.getManifestsDir();
        const manifestsPaths = z.array(z.string()).min(5).parse(this.getManifestsPathWithinDir(envDir));
        const exec = (cmd: string) => handleShellError(sh.exec(cmd, { silent: true })).stdout;

        // eslint-disable-next-line unicorn/no-array-reduce
        this.kubeObjectsAll = manifestsPaths.reduce<TKubeObjectAll[]>((acc, path, i) => {
            if (!path) return acc;

            console.log('Extracting kubeobject from manifest', i);

            const kubeObject = JSON.parse(exec(`cat ${path} | yq '.' -o json`)) as TKubeObjectAll;

            if (_.isEmpty(kubeObject)) return acc;
            // let's mutate to make it a bit faster and should be okay since we only do it here
            kubeObject.path = path;

            // Encode stringData into Data field for Secret Objects. This is to
            // ensure consistency and a single source of truth in handling the data.
            if (kubeObject.kind === 'Secret') {
                const encodedStringData = _.mapValues(kubeObject.stringData, (v) => {
                    return Buffer.from(String(v)).toString('base64');
                });

                kubeObject.data = ramda.mergeDeepRight(kubeObject.data ?? {}, encodedStringData);
            }

            const updatedPath = kubeObjectSchema.parse(kubeObject) as TKubeObjectAll;

            acc.push(updatedPath);
            return acc;
        }, []);

        return this;
    };

    /** Gets all the yaml manifests for an environment(local, staging etc)  */
    private getManifestsPathWithinDir = (environmentManifestsDir: string): string[] => {
        const manifestMatcher = '*ml';
        const allManifests = sh
            .exec(`find ${environmentManifestsDir} -name "${manifestMatcher}"`, {
                silent: true,
            })
            .split('\n');
        return allManifests;
    };

    getOfAKind = <K extends ResourceKind>(kind: K): TKubeObject<K>[] => {
        return (this.kubeObjectsAll as TKubeObject<K>[]).filter((o) => o.kind === kind);
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
        const secrets: TKubeObject<'Secret'>[] = this.getOfAKind('Secret').map((s) => {
            return {
                ...s,
                // Syncs all secrets
                selectedSecretsForUpdate: Object.keys(s?.data ?? {}),
            };
        }) as TKubeObject<'Secret'>[];
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

// const createKubeClass = () =>

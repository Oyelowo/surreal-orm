import fs from 'node:fs';
import yaml from 'yaml';
import { mergeUnsealedSecretToSealedSecret } from './sealedSecretsManager.js';
import { selectSecretKubeObjectsFromPrompt } from './secretsSelectorPrompter.js';
import sh from 'shelljs';
import * as R from 'ramda';
import path from 'node:path';
import _ from 'lodash';
import z from 'zod';
import { namespaceSchema } from '../../../src/infrastructure/namespaces/util.js';
import { getGeneratedEnvManifestsDir } from '../../../src/shared/directoriesManager.js';
import type { ResourceOutputDirProps } from '../../../src/shared/directoriesManager.js';
import { getResourceAbsolutePath } from '../../../src/shared/directoriesManager.js';
import type { Environment, ResourceName } from '../../../src/types/ownTypes.js';
import { generateManifests } from './generateManifests.js';
import { syncCrdsCode } from './syncCrdsCode.js';
import cliProgress from 'cli-progress';
import { PlainKubeBuildSecretsManager } from '../plainKubeBuildSecretsManager.js';

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
    /**
     * If path path format ==> kubernetes/generatedManifests/environment/resourceType/resource-name/1-manifest/kubeManifest.yaml
     *
     * resourceBaseDir ==> kubernetes/generatedManifests/environment/resourceType/resource-name
     */
    resourceBaseDir: z.string(),
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
export type TKubeObject<K extends ResourceKind = ResourceKind> = Extract<KubeObjectCustom, { kind: K }>;

export type TSecretKubeObject = TKubeObjectBaseCommonProps<'Secret'> & {
    selectedSecretsForUpdate?: string[] | null;
};

export class KubeObject {
    private kubeObjectsAll: TKubeObject[] = [];

    constructor(private environment: Environment) {
        this.kubeObjectsAll = this.syncAll().getAll();
    }

    getEnvironment = (): Environment => this.environment;

    getForApp = (outputDirectory: ResourceOutputDirProps['outputDirectory']): TKubeObject[] => {
        const resourceDir = getResourceAbsolutePath({
            outputDirectory,
            environment: this.environment,
        });
        return this.kubeObjectsAll.filter((m) => m.path.startsWith(`${resourceDir}${path.sep}`));
    };

    getAll = (): TKubeObject[] => {
        return this.kubeObjectsAll;
    };

    generateManifests = async (): Promise<void> => {
        PlainKubeBuildSecretsManager.syncAll();
        await generateManifests(this);
        this.syncAll();
        syncCrdsCode(this.getOfAKind('CustomResourceDefinition'));
    };

    /** Extract information from all the manifests for an environment(local, staging etc)  */
    private syncAll = (): this => {
        const envDir = getGeneratedEnvManifestsDir(this.environment);
        const manifestsPaths = this.getManifestsPathWithinDir(envDir);
        const bar1 = new cliProgress.SingleBar({}, cliProgress.Presets.shades_classic);

        console.log('Extracting kubeobject from manifest');
        bar1.start(manifestsPaths.length, 0);

        const kubeObjects: TKubeObject[] = [];
        manifestsPaths.forEach((manifestPath, i) => {
            if (!manifestPath) return;
            const kubeObject = yaml.parse(fs.readFileSync(manifestPath, 'utf8')) as TKubeObject;
            if (_.isEmpty(kubeObject)) throw new Error('Manifest is empty. Check the directory that all is well');

            // let's mutate to make it a bit faster and should be okay since we only do it here
            kubeObject.path = manifestPath;
            kubeObject.resourceBaseDir = path.join(path.dirname(manifestPath), '..');

            // Encode stringData into Data field for Secret Objects. This is to
            // ensure consistency and a single source of truth in handling the data.
            if (kubeObject.kind === 'Secret') {
                const encodedStringData = _.mapValues(kubeObject.stringData, (v): string =>
                    Buffer.from(v ?? '').toString('base64')
                );

                kubeObject.data = R.mergeDeepRight(kubeObject.data ?? {}, encodedStringData);
            }

            kubeObjects.push(kubeObjectSchema.parse(kubeObject) as TKubeObject);
            bar1.update(i);
        });

        this.kubeObjectsAll = kubeObjects;
        bar1.stop();

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
        return this.kubeObjectsAll.filter((o) => o.kind === kind) as TKubeObject<K>[];
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
        });
        mergeUnsealedSecretToSealedSecret({
            existingSealedSecretKubeObjects: this.getOfAKind('SealedSecret'),
            secretKubeObjects: secrets,
            onSealSecretValue: this.sealSecretValue,
        });

        // Sync kube object info after sealed secrets manifests have been updated
        this.syncAll();
    };

    /**
Sync only Sealed secrets that are selected from user in  the CLI terminal prompt. This is usually useful 
when you want to update Secrets in an existing cluster. Plain kubernetes
secrets should never be pushed to git but they help to generate sealed secrets.
*/
    syncSealedSecretsWithPrompt = async (): Promise<void> => {
        const selectedSecretObjects = await selectSecretKubeObjectsFromPrompt(this.getOfAKind('Secret'));
        mergeUnsealedSecretToSealedSecret({
            existingSealedSecretKubeObjects: this.getOfAKind('SealedSecret'),
            // Syncs only selected secrets from the CLI prompt
            secretKubeObjects: selectedSecretObjects,
            onSealSecretValue: this.sealSecretValue,
        });

        // Sync kube object info after sealed secrets manifests have been updated
        this.syncAll();
    };

    sealSecretValue({ namespace, name, secretValue }: { namespace: string; name: string; secretValue: string }) {
        const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = 'sealed-secrets';
        return sh
            .exec(
                `echo ${secretValue} | base64 - d | kubeseal--controller - name=${SEALED_SECRETS_CONTROLLER_NAME} \
            --raw --from - file=/dev/stdin --namespace ${namespace} \
            --name ${name}`
            )
            .stdout.trim();
    }
}

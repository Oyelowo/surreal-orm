import { SealedSecretTemplate } from '../../resources/types/sealedSecretTemplate';
import { getKubeResourceInfo, KubeObjectInfo } from './shared';
import p from 'path';
import yaml from 'js-yaml';
import sh from 'shelljs';
import { Environment, ResourceName } from '../../resources/types/own-types';
import _ from 'lodash';
import chalk from 'chalk';
import inquirer from 'inquirer';
import { Namespace } from '../../resources/infrastructure/namespaces/util';

const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = 'sealed-secrets';

/*
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
export async function syncAppSealedSecrets(environment: Environment) {
    const selectedUnsealedSecretsInfo = await prompSecretResourcesSelection(environment);

    for (let unsealedSecret of selectedUnsealedSecretsInfo) {
        const sealedSecretInfo = getKubeResourceInfo({ kind: "SealedSecret", environment });

        mergeUnsealedSecretToSealedSecret({
            unsealedSecretInfo: unsealedSecret,
            sealedSecretInfo: sealedSecretInfo,
        });
    }
}

type MergeProps = {
    unsealedSecretInfo: KubeObjectInfo;
    sealedSecretInfo: KubeObjectInfo[];
};

export function mergeUnsealedSecretToSealedSecret({ sealedSecretInfo, unsealedSecretInfo }: MergeProps): void {
    const {
        stringData,
        data,
        metadata: { name, namespace /* annotations */ },
    } = unsealedSecretInfo;

    if (!name && namespace) {
        throw new Error('Name and namespace not provided in the secret');
    }

    // Get corresponding previously(if it exists) generated sealed secrets info.
    const matchesUnsealedSecret = ({ metadata: m }: KubeObjectInfo): boolean =>
        m.name === name && m.namespace === namespace;
    const existingSealedSecretJsonData = sealedSecretInfo?.find(matchesUnsealedSecret);

    const sealSecretValue = (secretValue: string): string => {
        return sh
            .exec(
                `echo -n ${secretValue} | kubeseal --controller-name=${SEALED_SECRETS_CONTROLLER_NAME} \
            --raw --from-file=/dev/stdin --namespace ${namespace} \
            --name ${name}`
            )
            .stdout.trim();
    };

    const dataToSeal = stringData ?? data ?? {};
    const filteredSealedSecretsData = _.mapValues(dataToSeal, sealSecretValue) as unknown as Record<
        string,
        string | null
    >;

    // Update sealed secret object to be converted to yaml
    const updatedSealedSecrets: SealedSecretTemplate = {
        // For some reason, typescript is not detecting the correct type here.
        kind: 'SealedSecret',
        apiVersion: 'bitnami.com/v1alpha1',
        metadata: {
            name: unsealedSecretInfo.metadata.name,
            namespace: unsealedSecretInfo.metadata.namespace,
            annotations: {
                'sealedsecrets.bitnami.com/managed': 'true',
                ...existingSealedSecretJsonData?.metadata.annotations,
            },
            ...existingSealedSecretJsonData?.metadata,
        },
        spec: {
            encryptedData: {
                ...existingSealedSecretJsonData?.spec?.encryptedData,
                ...filteredSealedSecretsData,
            },
            template: {
                ...existingSealedSecretJsonData?.spec?.template,
                data: null,
                metadata: unsealedSecretInfo.metadata,
                type: unsealedSecretInfo.type,
            },
        },
    };

    // GET SEALED SECRET PATH USING UNSEALED SECRET PATH
    const appManifestsDir = p.dirname(unsealedSecretInfo.path);
    // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
    // and we want as basedir: kubernetes/manifests/generated/production/applications/graphql-mongo
    const appBaseDir = p.join(appManifestsDir, '..');
    const sealedSecretDir = p.join(appBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
    sh.mkdir(sealedSecretDir);
    const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-secret-${name}-${namespace}.yaml`);

    sh.exec(`echo '${yaml.dump(updatedSealedSecrets)}' > ${sealedSecretFilePath}`);
}

/* 
CLI tool
Prompts the user to select secret resources in various namespaces to update.
After, the first prompt, user has to input the exact secret keys of a resource
they want to update
*/
async function prompSecretResourcesSelection(environment: Environment): Promise<KubeObjectInfo[]> {
    // Gets all secrets sorting the secret resources in applications namespace first
    const originalSecretsInfo = _.sortBy(getKubeResourceInfo({ kind: 'Secret', environment }), [
        (a) => a.metadata.namespace !== 'applications',
    ]);
    const sercretObjectsByNamespace = _.groupBy(originalSecretsInfo, (d) => d.metadata.namespace);

    // Name and value have to be defined for inquirer if not using basic string
    const mapToPrompterValues = (secret: KubeObjectInfo): { name: string; value: KubeObjectInfo } => ({
        name: secret?.metadata?.name,
        value: secret,
    });

    /* 
    Create a list of applications separated/grouped by their namespaces
    e.g 
    Namespace  ===> application
         - service 1
         - service 2
         - service 3
    Namespace  ===> infra
         - infra 1
         - infra 2
     */
    const applicationList = Object.entries(sercretObjectsByNamespace).flatMap(([namespace, namespaceSecretObjects]) => [
        new inquirer.Separator(),
        new inquirer.Separator(`Namespace ==> ${namespace} `),
        ...namespaceSecretObjects.map(mapToPrompterValues),
    ]);

    const allResourceAnswerKeyName = 'selectedSecretResources';
    const { selectedSecretResources } = await inquirer.prompt<{ [allResourceAnswerKeyName]: KubeObjectInfo[] }>({
        type: 'checkbox',
        message: 'Select resource secret you want to update',
        name: allResourceAnswerKeyName,
        choices: applicationList,
        validate(answer) {
            if (answer.length < 1) {
                return 'You must choose at least one secret.';
            }

            return true;
        },
        pageSize: 2000,
    });

    const allSelected = originalSecretsInfo.length === selectedSecretResources.length;
    if (allSelected) {
        return originalSecretsInfo;
    }

    // List of secrets keys/names in each kube secret resource.
    // e.g for react-app: [secretKey1, secretKey2, ...]
    const appSecretKeysByNamespace = await promptSecretKeysSelection(selectedSecretResources);

    return filterSecrets(selectedSecretResources, appSecretKeysByNamespace);
}

type AppName = ResourceName | string;
type SecretKey = string;
type AppSecretKeysByNamespace = Record<Namespace, Record<AppName, SecretKey[]>>;

async function promptSecretKeysSelection(allSecretResources: KubeObjectInfo[]): Promise<AppSecretKeysByNamespace> {
    const createAppSecretSelectionPrompt = (resource: KubeObjectInfo) => {
        const { name, namespace } = resource.metadata;
        const secrets = resource.stringData ?? resource.data ?? {};
        const secretKeys = Object.keys(secrets);

        return {
            type: 'checkbox',
            name: `${namespace}.${name}`,
            message: chalk.cyanBright(`Select secret - ${name} from the namespace - ${namespace}`),
            choices: secretKeys.flatMap((secretKey) => [new inquirer.Separator(), secretKey]),
            pageSize: 2000,
        };
    };

    return await inquirer.prompt<AppSecretKeysByNamespace>(allSecretResources.map(createAppSecretSelectionPrompt));
}

function filterSecrets(
    allSecretResources: KubeObjectInfo[],
    selectedSecrets: AppSecretKeysByNamespace
): KubeObjectInfo[] {
    return allSecretResources?.map((info) => {
        const { name, namespace } = info.metadata;
        const { stringData, data } = info;

        if (!namespace) {
            throw new Error(`namespace missing for ${name}`);
        }

        if (!stringData && !data) {
            throw new Error('data or stringData field missing in secret Resource');
        }

        const secretDataKeyName = stringData ? 'stringData' : 'data';
        const secretRecord = info[secretDataKeyName] ?? {};

        const selectedSecretsKeys = selectedSecrets[namespace][name];
        let filteredSecretRecords = _.pickBy(secretRecord, (_v, k) => selectedSecretsKeys.includes(k));

        return {
            ...info,
            [secretDataKeyName]: filteredSecretRecords,
        };
    });
}

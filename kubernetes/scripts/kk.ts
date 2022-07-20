import { Namespace } from './../resources/infrastructure/namespaces/util';
import inquirer from 'inquirer';
import _ from 'lodash';
import { getSecretResourceInfo, KubeObjectInfo } from './utils/shared';
// import { getKubeResourceTypeInfo } from "./shared"
// getKubernetesManifestInfo
import chalk from 'chalk';
import sh from 'shelljs';
import { Environment, ResourceName } from '../resources/types/own-types';



getSelectedSecretKeysFromPrompt("local").then(c=> console.log("xxx", c))

async function getSelectedSecretKeysFromPrompt(environment: Environment): Promise<KubeObjectInfo[]> {
    // Gets all secrets sorting the secret resources in applications namespace first
    const secretsInfo = _.sortBy(getSecretResourceInfo(environment), [(a) => a.metadata.namespace !== 'applications']);
    const sercretObjectsByNamespace = _.groupBy(secretsInfo, (d) => d.metadata.namespace);

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

    const allResourceAnswerKeyName = 'allSecretResources';
    const { allSecretResources } = await inquirer.prompt<{ [allResourceAnswerKeyName]:KubeObjectInfo[]}>(
        {
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
        },
    );

    // List of secrets keys/names in each kube secret resource.
    // e.g for react-app: [secretKey1, secretKey2, ...]
    const appSecretKeysByNamespace = await promptSecretKeysSelection(allSecretResources);

    return filterSecrets(allSecretResources, appSecretKeysByNamespace);
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
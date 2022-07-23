import { TKubeObject, TSecretKubeObject } from './kubeObject';
import { ResourceName } from '../../../resources/types/own-types';
import _ from 'lodash';
import chalk from 'chalk';
import inquirer from 'inquirer';
import { Namespace } from '../../../resources/infrastructure/namespaces/util';

type AppName = ResourceName | string;
type SecretKey = string;
type AppSecretKeysWithinNamespaces = Record<Namespace, Record<AppName, SecretKey[]>>;

/**  Cli command that prompts user to select from a list of applications separated/grouped by their namespaces
 * After, the first prompt, user has to input the exact secret keys of a resource they want to update
 * @example
 * Prompt "Which of the secrets do you want to update?"
 *  Namespace  ===> application
         - service 1
         - service 2
         - service 3
    Namespace  ===> infra
         - infra 1
         - infra 2
 **/
export async function selectSecretKubeObjectsFromPrompt(secretKubeObjects: TSecretKubeObject[]): Promise<TSecretKubeObject[]> {
    // We want applications secrets first
    const secretKubeObjectsSorted = _.sortBy(secretKubeObjects, [(a) => a.metadata.namespace !== 'applications']);
    const sercretObjectsByNamespace = _.groupBy(secretKubeObjects, (d) => d.metadata.namespace);

    // Name and value have to be defined for inquirer if not using basic string
    const mapToPrompterValues = (secret: TKubeObject): { name: string; value: TKubeObject } => ({
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
    const { selectedSecretResources: selectedSecretObjects } = await inquirer.prompt<{
        [allResourceAnswerKeyName]: TSecretKubeObject[];
    }>({
        type: 'checkbox',
        message: 'Which of the secrets do you want to update?',
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

    const allSelected = secretKubeObjects.length === selectedSecretObjects.length;
    if (allSelected) {
        return secretKubeObjects;
    }

    const selectedSecretsDataKeys = await promptSecretDataSelection(selectedSecretObjects);

    return pickBySelectedSecretDataKeys(selectedSecretObjects, selectedSecretsDataKeys);
}

/** Creates a list of Command line prompts that appear one after the other for
 *  selecting from data/StringData field of each Secret object.
 *
 * e.g if a kubernetes Secret object(oyelowo-app) in `application` namespace has secret data: { USERNAME: "xxx", PASSWORD: "123" },
 * it will list them as below and do that for other app Secrets as well:
 * @example Example of a Command line prompt
 
* "Select secrets from oyelowo-app in the application namespace"
 * USERNAME
 * PASSWORD
 * ...
 */
async function promptSecretDataSelection(
    secretObjectsInfo: TSecretKubeObject[]
): Promise<AppSecretKeysWithinNamespaces> {
    const createAppSecretDataSelectionPrompt = (resource: TSecretKubeObject) => {
        const { name, namespace } = resource.metadata;
        const secrets = resource.stringData ?? resource.data ?? {};
        const secretKeys = Object.keys(secrets);

        return {
            type: 'checkbox',
            name: `${namespace}.${name}`,
            message: chalk.cyanBright(`Select secrets from  ${name} in the ${namespace} namespace`),
            choices: secretKeys.flatMap((secretKey) => [new inquirer.Separator(), secretKey]),
            pageSize: 2000,
        };
    };

    // We want want many prompts which will appear one after the other
    return await inquirer.prompt<AppSecretKeysWithinNamespaces>(
        secretObjectsInfo.map(createAppSecretDataSelectionPrompt)
    );
}

/**
 * Picks out the selected Secret data key from within
 * all Secret kubeObjectInfo
 * @example
 * # For each of the Secret kubeObjectInfos(like below), you can pick out "USERNAME" & "PASSWORD"
 * # Before ....
 * Kind: Secret,
 * metadata: { name: "oyelowo-app", namespace: "application"},
 * data: {
 *    USERNAME: "oyelowo"
 *    PASSWORD: "123"
 *    CLIENT_ID: "itrirt"
 *    CLIENT_SECRET: "mmomonienre",
 * }
 *
 * # After ....
 * Kind: Secret,
 * metadata: { name: "oyelowo-app", namespace: "application"},
 * data: {
 *    USERNAME: "oyelowo"
 *    PASSWORD: "123"
 * }
 * ...
 * */
function pickBySelectedSecretDataKeys(
    secretObjects: TSecretKubeObject[],
    secretDataKeys: AppSecretKeysWithinNamespaces
): TSecretKubeObject[] {
    return secretObjects?.map((info) => {
        const { name, namespace } = info.metadata;
        const { stringData, data } = info;

        if (!namespace) {
            throw new Error(`namespace missing for ${name}`);
        }

        if (!stringData && !data) {
            throw new Error('data or stringData field missing in secret Resource');
        }

        const secretDataKeyName = stringData ? 'stringData' : 'data';
        const secretData = info[secretDataKeyName] ?? {};
        const selectedSecretDataKeys = secretDataKeys[namespace][name];

        const filteredSecretRecords = _.pickBy(secretData, (_v, k) => selectedSecretDataKeys.includes(k));

        return {
            ...info,
            [secretDataKeyName]: filteredSecretRecords,
        };
    });
}

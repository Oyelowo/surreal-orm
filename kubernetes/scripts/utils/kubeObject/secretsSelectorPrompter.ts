import type { TKubeObject } from './kubeObject.js';
import type { ResourceName } from '../../../src/resources/types/ownTypes.js';
import _ from 'lodash';
import chalk from 'chalk';
import inquirer from 'inquirer';
import { Namespace } from '../../../src/resources/infrastructure/namespaces/util.js';

type SecretKey = string;
type AppSecretKeysWithinNamespaces = Record<Namespace, Record<ResourceName | string, SecretKey[]>>;

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
export async function selectSecretKubeObjectsFromPrompt(
    secretKubeObjects: TKubeObject<'Secret'>[]
): Promise<TKubeObject<'Secret'>[]> {
    // We want secrets in applications namesapce first
    const secretKubeObjectsSorted = _.sortBy(secretKubeObjects, [(s) => s.metadata.namespace !== 'applications']);
    const sercretObjectsByNamespace = _.groupBy(secretKubeObjectsSorted, (s) => s.metadata.namespace);

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

    const promptResponse = await inquirer.prompt<{
        selectedSecretObjects: TKubeObject<'Secret'>[];
    }>({
        type: 'checkbox',
        message: 'Which of the secrets do you want to update?',
        name: 'selectedSecretObjects',
        choices: applicationList,
        validate(answer) {
            console.log("Error" + JSON.stringify(answer.length));
            
            if (answer.length === 0) {
                return 'You must choose at least one secret.';
            }

            return true;
        },
        pageSize: 2000,
    });

    const secretkeysData = await promptSecretObjectDataSelection(promptResponse.selectedSecretObjects);

    return secretKubeObjects?.map((s) => {
        const { name, namespace } = s?.metadata ?? {};
        if (!namespace || !name) {
            throw new Error('Namespace not found in secret');
        }

        return {
            ...s,
            selectedSecretsForUpdate: secretkeysData?.[namespace]?.[name] ?? [],
        };
    }).filter(s=>s.selectedSecretsForUpdate.length > 0);
}

/** Creates a list of Command line prompts that appear one after the other for
 *  selecting from data field of each Secret object.
 *
 * e.g if a kubernetes Secret object(oyelowo-app) in `application` namespace has secret data: { USERNAME: "xxx", PASSWORD: "123" },
 * it will list them as below and do that for other app Secrets as well:
 * @example Example of a Command line prompt
 
* "Select secrets from oyelowo-app in the application namespace"
 * USERNAME
 * PASSWORD
 * ...
 */
async function promptSecretObjectDataSelection(
    secretKubeObjects: TKubeObject<'Secret'>[]
): Promise<AppSecretKeysWithinNamespaces> {
    const createAppSecretDataSelectionPrompt = (resource: TKubeObject<'Secret'>) => {
        const { name, namespace } = resource.metadata;
        const secretKeys = Object.keys(resource.data);
        const promptKey = `${namespace}.${name}`;
        return {
            type: 'checkbox',
            name: promptKey,
            message: chalk.cyanBright(`Select secrets from  ${name} in the ${namespace} namespace`),
            choices: secretKeys.flatMap((secretKey) => [new inquirer.Separator(), secretKey]),
            pageSize: 2000,
        };
    };

    // We many prompts which for all secrets in various namepsaces appear one after the other
    return await inquirer.prompt<AppSecretKeysWithinNamespaces>(
        secretKubeObjects.map(createAppSecretDataSelectionPrompt)
    );
}

import { Namespace } from './../resources/infrastructure/namespaces/util';
import inquirer from 'inquirer';
import _ from 'lodash';
import { getSecretResourceInfo, KubeObjectInfo } from './utils/shared';
// import { getKubeResourceTypeInfo } from "./shared"
// getKubernetesManifestInfo
import chalk from 'chalk';
import sh from 'shelljs';
import { clearPlainInputTsSecretFilesContents, syncSecretsTsFiles } from './secretsManagement/syncSecretsTsFiles';
import { generateManifests } from './utils/generateManifests';
import { getImageTagsFromDir } from './utils/getImageTagsFromDir';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { generateAllSealedSecrets } from './utils/sealed-secrets/generateAllSealedSecrets';
import { promptSecretsKeepingConfirmations } from './utils/sealed-secrets/promptSecretsKeepingConfirmations';
// import { promptSecretsKeepingConfirmations } from './utils/promptSecretsKeepingConfirmations';
import { updateAppSealedSecrets } from './utils/sealed-secrets/updateApplicationsSecrets';
// import { getSecretManifestsPaths, } from './utils/sealedSecrets';
import { getSecretManifestsPaths, promptEnvironmentSelection } from './utils/shared';
// import { updateAppSealedSecrets } from './utils/updateApplicationsSecrets';

async function main() {
    const { generateSecretsOptions } = await promptSealedSecretsMergingOptions();

    const { environment } = await promptEnvironmentSelection();
    await promptKubernetesClusterSwitch(environment);
    const { keepPlainSecretsInput, keepUnsealedSecretManifestsOutput } = await promptSecretsKeepingConfirmations();

    const imageTags = await getImageTagsFromDir();

    await generateManifests({
        environment,
        imageTags,
    });

    syncSecretsTsFiles();

    if (generateSecretsOptions === 'Generate all secrets') {
        generateAllSealedSecrets({ environment });
    } else if (generateSecretsOptions === 'merge self managed secrets') {
        updateAppSealedSecrets(environment);
    }

    if (!keepPlainSecretsInput) {
        clearPlainInputTsSecretFilesContents();
    }

    if (!keepUnsealedSecretManifestsOutput) {
        const removeSecret = (path: string) => sh.rm('-rf', path);
        getSecretManifestsPaths(environment).forEach(removeSecret);
    }
}

// main();

export async function promptSealedSecretsMergingOptions() {
    const options = ['merge self managed secrets', 'Generate all secrets'] as const;
    const choices = options.flatMap((env) => [env, new inquirer.Separator()]);
    const optionName = 'generateSecretsOptions';

    const answers = await inquirer.prompt<Record<typeof optionName, typeof options[number]>>({
        type: 'list',
        name: optionName,
        message: chalk.blueBright`Sealed secret generation options‼️‼️‼️‼️`,
        choices,
        default: options[0],
    });

    return answers;
}

async function mainn() {
    // Gets all secrets putting the resources in applications namespace first
    const secretsInfo = _.sortBy(getSecretResourceInfo('local'), [(a) => a.metadata.namespace !== 'applications']);
    const secretsInfoByNamespace = _.groupBy(secretsInfo, (d) => d.metadata.namespace);

    const mapToPrompterValues = (o: KubeObjectInfo): { name: string; value: KubeObjectInfo } => ({
        name: o?.metadata?.name,
        value: o,
    });

    // Create a list of applications separated/grouped by their namespaces
    const applicationList = Object.entries(secretsInfoByNamespace).flatMap(([namespace, v]) => [
        new inquirer.Separator(),
        new inquirer.Separator(`Namespace ==> ${namespace} `),
        ...v.map(mapToPrompterValues),
    ]);

    const answers = await inquirer.prompt<{ resources: KubeObjectInfo[] }>([
        {
            type: 'checkbox',
            message: 'Select resource secret you want to update',
            name: 'resources',
            choices: applicationList,
            validate(answer) {
                if (answer.length < 1) {
                    return 'You must choose at least one topping.';
                }

                return true;
            },
            pageSize: 2000,
        },
    ]);

    console.log('outerxxx', JSON.stringify(answers, null, '  '));

    // List of secrets in each kube secret resource.
    // e.g for react-app: [secretKey1, secretKey2, ...]
    const applicationsSecretKeys: Parameters<typeof inquirer.prompt>[0] = answers.resources.map(
        ({ metadata: { name, namespace }, data, stringData }) => ({
            type: 'checkbox',
            name: `${namespace}.${name}`,
            message: chalk.cyanBright(`Select secret - ${name} from the namespace - ${namespace}`),
            choices: Object.entries(data ?? stringData ?? {}).flatMap(([secretKey, _]) => [
                new inquirer.Separator(),
                { name: secretKey },
            ]),
            pageSize: 2000,
        })
    );

    type AppName = string;
    type SecretKey = string;
    type AppSecretKeysByNamespace = Record<Namespace, Record<AppName, SecretKey[]>>;
    const appSecretKeysByNamespace = await inquirer.prompt<AppSecretKeysByNamespace>(applicationsSecretKeys);

    const getSelectedSecretKeysFromPrompt = ({ namespace, resourceName }: { namespace: Namespace; resourceName: string }) =>
        appSecretKeysByNamespace[namespace][resourceName];

    const filteredSecretObject = answers?.resources?.map((info) => {
        console.log("infoffff", info)
        if (!info.stringData && !info.data) {
            throw new Error("secrets not present")
        }
        const secretDataKeyName = info.stringData ? 'stringData' : 'data';
        const secretRecord = info[secretDataKeyName] ?? {};

        const selectedSecretsKeys = getSelectedSecretKeysFromPrompt({
            namespace: info.metadata.namespace,
            resourceName: info.metadata.name,
        });

        let filteredSecretRecords = Object.entries(secretRecord).filter(([secName, _secValue]) => selectedSecretsKeys.includes(secName));
        return {
            ...info,
            [secretDataKeyName]: Object.fromEntries(filteredSecretRecords),
        };
    });
    console.log('inner', JSON.stringify(filteredSecretObject, null, '  '));
}

mainn();

import chalk from 'chalk';
import inquirer from 'inquirer';
import _ from 'lodash';
import sh from 'shelljs';
import { ingressControllerPorts, INGRESS_EXTERNAL_PORT_LOCAL } from '../../src/infrastructure/ingress/hosts.js';
import { Environment } from '../../src/types/ownTypes.js';

const switchToCluster = (name: string) => {
    const selectContext = sh.exec(`kubectl config use-context ${name}`, { silent: true });
    sh.echo(chalk.greenBright(`${selectContext.stdout} 🎉`));
};

async function createCluster(clusterChoices: string[], likelyLocalCluster: (s: string) => boolean) {
    const getSufficedNumber = (str: string) => str.match(/\d+$/);
    const newClusterDefaultNumberSuffix = _.chain(clusterChoices)
        .filter(likelyLocalCluster)
        .map(String)
        .map(getSufficedNumber)
        .map(Number)
        .max()
        .add(1)
        .value();

    const DEFAULT_CLUSTER_NAME = `local-${newClusterDefaultNumberSuffix}`;

    const newClusterNameKey = 'newClusterName';
    const answerNewCluster = await inquirer.prompt<Record<typeof newClusterNameKey, boolean>>([
        {
            type: 'input',
            name: newClusterNameKey,
            message: chalk.blueBright(`What do you want to name your cluster? Press Enter to use default`),
            default: DEFAULT_CLUSTER_NAME,
        },
    ]);

    const { http, https } = ingressControllerPorts;
    sh.exec(
        `k3d cluster create ${answerNewCluster.newClusterName} --port ${INGRESS_EXTERNAL_PORT_LOCAL}:${http}@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"`
    );
    // Uncomment if you also want secure port at 8443
    // sh.exec(
    //     `k3d cluster create ${clusterName} --port ${INGRESS_EXTERNAL_PORT_LOCAL}:${http}@loadbalancer --port 8443:${https}@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"`
    // );
    switchToCluster(`k3d-${answerNewCluster.newClusterName}`);
}

const LOCAL_CLUSTER_REGEX = /local|k3d|k3d-local|minikube/g;
const likelyLocalCluster = (s: string) => LOCAL_CLUSTER_REGEX.test(s);

export function getClustersList() {
    const kubernetesContexts = sh.exec('kubectl config get-contexts --output=name', { silent: true });
    const clusters = kubernetesContexts.stdout.trim().split('\n');
    const clusterChoices = _.sortBy(clusters, [likelyLocalCluster]);
    return clusterChoices;
}

/*
Prompt cluster selection
*/
export async function promptKubernetesClusterSwitch(environment: Environment) {
    const clusterChoices = getClustersList();

    const clusterChoicesWithSeparators = clusterChoices.flatMap((context) => [context, new inquirer.Separator()]);

    const createNewLocalClusterOption = 'Create a new local cluster instead?';

    const name = 'cluster';
    const answers: Record<typeof name, string> = await inquirer.prompt([
        {
            type: 'list',
            name,
            message: chalk.greenBright(`🆘Select the ${environment.toLocaleUpperCase()} cluster ‼️‼️‼️‼️`),
            choices: [
                new inquirer.Separator(),
                createNewLocalClusterOption,
                new inquirer.Separator(),
                ...clusterChoicesWithSeparators,
            ],
            default: clusterChoices.find((element) => likelyLocalCluster(element)),
            pageSize: 20,
        },
    ]);

    if (answers.cluster !== createNewLocalClusterOption) {
        switchToCluster(answers.cluster);
        return;
    }

    // Create cluster
    await createCluster(clusterChoices, likelyLocalCluster);
}

import chalk from 'chalk';
import inquirer from 'inquirer';
import sh from 'shelljs';
import { getClustersList } from './utils/promptKubernetesClusterSwitch.js';

const deleteLocalCluster = (name: string) => sh.exec(`minikube delete -p ${name}`);

async function main() {
    const clusterNames = 'clusterNames';
    const clusters = getClustersList();

    const answer = await inquirer.prompt<Record<typeof clusterNames, string[]>>({
        type: 'checkbox',
        name: clusterNames,
        message: chalk.blueBright(`Select the clusters you want to delete`),
        choices: clusters,
    });

    answer.clusterNames.forEach(deleteLocalCluster);
}

await main();

import chalk from 'chalk';
import inquirer from 'inquirer';
import sh from 'shelljs';
import { getClustersList } from './utils/promptKubernetesClusterSwitch';

async function main() {
    const clusterNames = 'clusterNames';
    const clusters = getClustersList();

    const answer = await inquirer.prompt<Record<typeof clusterNames, string[]>>({
        type: 'checkbox',
        name: clusterNames,
        message: chalk.blueBright(`Select the clusters you want to delete`),
        choices: clusters,
    });

    // starts with k3d-<clustername>
    const deleteLocalCluster = (name: string) => sh.exec(`k3d cluster delete ${name.slice(4)}`);

    answer.clusterNames.forEach(deleteLocalCluster);
}

main();

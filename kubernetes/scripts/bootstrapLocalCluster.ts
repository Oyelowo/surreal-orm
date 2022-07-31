import sh from 'shelljs';
import { promptKubernetesClusterSwitch } from './utils/promptKubernetesClusterSwitch';
import { setupCluster } from './utils/setupCluster';
import chalk from 'chalk';
import inquirer from 'inquirer';

/* 
Creates and bootstraps cluster
*/

async function main() {
    const { shouldRebuild, clusterRefreshMode } = await promptQuestions();
    await promptKubernetesClusterSwitch('local');

    const trigger = clusterRefreshMode === 'live' ? '' : '--trigger="manual"';

    if (!shouldRebuild) {
        return;
    }

    await setupCluster('local');

    sh.exec(`skaffold dev --port-forward --cleanup=false  ${trigger}  --no-prune=true --no-prune-children=true`);
}

main().catch((e) => `Falied to bootstrap. Error: ${e}`);

async function promptQuestions() {
    const clusterRefreshMode = 'clusterRefreshMode';
    const regenerateKubernetesManifests = 'regenerateKubernetesManifests';
    const shouldRebuild = 'shouldRebuild';
    type Key = typeof clusterRefreshMode | typeof regenerateKubernetesManifests | typeof shouldRebuild;

    const manualTrigger = 'Manual Trigger (Requires keyboard input from the termial)';
    const clusterRefreshModesChoices = ['live', manualTrigger] as const;
    type ClusterMode = typeof clusterRefreshModesChoices[number];
    const defaultTriggerMode: ClusterMode = 'live';

    type Prompt = {
        [shouldRebuild]: boolean;
        [clusterRefreshMode]: ClusterMode;
    };
    const answers = await inquirer.prompt<Prompt>([
        {
            type: 'confirm',
            name: shouldRebuild,
            message: chalk.blueBright(`Do you want to rebuild?`),
            default: false,
        },
        {
            type: 'list',
            name: clusterRefreshMode,
            choices: clusterRefreshModesChoices,
            message: chalk.blueBright('Choose cluster Refresh Mode ‼️‼️‼️‼️'),
            default: defaultTriggerMode,
        },
    ]);

    return answers;
}

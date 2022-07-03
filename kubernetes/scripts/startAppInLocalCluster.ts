import { ingressControllerPorts } from '../resources/infrastructure/ingress/hosts';
import chalk from 'chalk';
import inquirer from 'inquirer';
import sh from 'shelljs';
import yargs from 'yargs';
import { INGRESS_EXTERNAL_PORT_LOCAL } from '../resources/infrastructure/ingress/hosts';

// This autoreloads the app but waits for user inputs
async function startAppInLocalCluster() {
    const { deleteExistingCluster, regenerateKubernetesManifests, clusterName } = await promptQuestions();
    const { http, https } = ingressControllerPorts;

    if (deleteExistingCluster) {
        sh.exec(`k3d cluster delete ${clusterName}`);
    }

    sh.exec(
        `k3d cluster create ${clusterName} --port ${INGRESS_EXTERNAL_PORT_LOCAL}:${http}@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"`
    );
    // Uncomment if you also want secure port at 8443
    // sh.exec(
    //     `k3d cluster create ${clusterName} --port ${INGRESS_EXTERNAL_PORT_LOCAL}:${http}@loadbalancer --port 8443:${https}@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"`
    // );
    sh.exec(`kubectx k3d-${clusterName}`);

    if (regenerateKubernetesManifests) {
        sh.exec(`make generate_manifests_ci environment=local`);
    }

    // Scaffold should wait for user input before reloading (--trigger="manual"). Without this, it hot reloads
    sh.exec(`skaffold dev --port-forward --cleanup=false`);
    // sh.exec(`skaffold dev --cleanup=false  --trigger="manual"  --no-prune=true --no-prune-children=true`);
}

startAppInLocalCluster();

async function promptQuestions() {
    const DEFAULT_CLUSTER_NAME = 'local';
    const deleteExistingCluster = 'deleteExistingCluster';
    const regenerateKubernetesManifests = 'regenerateKubernetesManifests';
    const clusterName = 'clusterName';
    type Key = typeof deleteExistingCluster | typeof regenerateKubernetesManifests | typeof clusterName;

    const answers = await inquirer.prompt<Record<Key, boolean>>([
        {
            type: 'input',
            name: clusterName,
            message: chalk.blueBright(`What do you want to name your cluster? Press Enter to use default`),
            default: DEFAULT_CLUSTER_NAME,
        },
        {
            type: 'confirm',
            name: deleteExistingCluster,
            message: chalk.blueBright('🆘Do you want to delete the existing local cluster??? ‼️‼️‼️‼️'),
            default: false,
        },
        {
            type: 'confirm',
            name: regenerateKubernetesManifests,
            message: chalk.blueBright(
                `🆘Would you like to regenerate/update Kubernetes manifests from code?? ‼️‼️‼️‼️`
            ),
            default: false,
        },
    ]);

    return answers;
}

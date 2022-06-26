import { ingressControllerPorts } from '../resources/infrastructure/ingress/hosts';
import chalk from 'chalk';
import inquirer from 'inquirer';
import sh from 'shelljs';
import yargs from 'yargs';
import { INGRESS_EXTERNAL_PORT_LOCAL } from '../resources/infrastructure/ingress/hosts';

export const ARGV = yargs(process.argv.slice(2))
    .options({
        environment: {
            alias: 'n',
            choices: ["name"],
            describe: "The environment you're generating the manifests for.",
            demandOption: true,
        },
    })
    .parseSync();


const CLUSTER_NAME = "local-cluster-34i2jn23j"


// This autoreloads the app but waits for user inputs
async function startAppInLocalCluster() {
    const { deleteExistingCluster, regenerateKubernetesManifests } = await promptQuestions();
    const { http, https } = ingressControllerPorts;

    if (deleteExistingCluster) {
        sh.exec(`k3d cluster delete ${CLUSTER_NAME}`);
    }

    sh.exec(`k3d cluster create ${CLUSTER_NAME} --port ${INGRESS_EXTERNAL_PORT_LOCAL}:${http}@loadbalancer --port 8443:${https}@loadbalancer --k3s-arg "--no-deploy=traefik@server:*"`)
    sh.exec(`kubectx k3d-${CLUSTER_NAME}`);

    if (regenerateKubernetesManifests) {
        sh.exec(`make generate_manifests_ci environment=local`);
    }

    sh.exec(`make format`);
    // sh.exec(`kubectl apply -R -f  ./manifests/local/secrets-encrypted`);

    // Scaffold should wait for user input before reloading (--trigger="manual"). Without this, it hot reloads
    sh.exec(`skaffold dev --cleanup=false  --trigger="manual"  --no-prune=true --no-prune-children=true`);

    // This only runs once
    // sh.exec(`skaffold run --trigger="manual" --no-prune=true --no-prune-children=true`);

}

startAppInLocalCluster()

async function promptQuestions() {
    const deleteExistingCluster = "deleteExistingCluster";
    const regenerateKubernetesManifests = "regenerateKubernetesManifests";
    type Key = typeof deleteExistingCluster | typeof regenerateKubernetesManifests;

    const answers = await inquirer.prompt<Record<Key, boolean>>([
        {
            type: 'confirm',
            name: deleteExistingCluster,
            message: chalk.blueBright(
                '🆘Do you want to keep delete the existing local cluster??? ‼️‼️‼️‼️'
            ),
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

    return answers
}


import c from 'chalk';
import path from 'node:path';
import sh from 'shelljs';
import { getResourceAbsolutePath } from '../../shared/directoriesManager.js';
import { Environment } from '../../types/ownTypes.js';
import { namespaces } from '../namespaces/util.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';
import { PlainKubeBuildSecretsManager } from '../../../scripts/utils/plainKubeBuildSecretsManager.js';

const DOCKER_SERVER = 'ghcr.io';
export const DOCKER_REGISTRY_KEY = 'my-registry-key';

const env = getEnvVarsForKubeManifests();
const secrets = new PlainKubeBuildSecretsManager('infrastructure', 'argocd', env.ENVIRONMENT).getSecrets();
const DOCKER_USERNAME = secrets.CONTAINER_REGISTRY_USERNAME;
const DOCKER_PASSWORD = secrets.CONTAINER_REGISTRY_PASSWORD;

// Create secret for argocd to be able to access repo where docker images are stored
export function createContainerRegistrySecret(environment: Environment): void {
    const dir = path.join(
        getResourceAbsolutePath({
            outputDirectory: 'infrastructure/argocd-applications-parents',
            environment,
        }),
        '1-manifest'
    );
    const file = path.join(dir, 'secret-docker-registry.yaml');

    if (!DOCKER_USERNAME || !DOCKER_PASSWORD) {
        console.warn(c.bgYellowBright('docker username nor password not provideed'));
        return;
    }

    sh.mkdir(dir);
    sh.touch(file);

    sh.exec(`
  kubectl create secret docker-registry ${DOCKER_REGISTRY_KEY} --docker-server=${DOCKER_SERVER} \
     --docker-username=${DOCKER_USERNAME} --docker-password=${DOCKER_PASSWORD} --namespace=${namespaces.applications} \
     -o yaml --dry-run=client > ${file}`);
}

// interface DockerRawData {
//     auths: {
//         'ghrc.io': { username: string; password: string; auth: string };
//     };
// }

// const DOCKER_USERNAME = env.username
// const DOCKER_PASSWORD = env.password
// 'argocd',
//     environment
//     ).getSecretsForResource();
// const dataRaw: DockerRawData = {
//     auths: {
//         'ghrc.io': {
//             username: DOCKER_USERNAME,
//             password: DOCKER_PASSWORD,
//             auth: toBase64(`${DOCKER_USERNAME}:${DOCKER_PASSWORD}`),
//         },
//     },
// };
// function toBase64(text: string) {
//     return Buffer.from(text).toString('base64');
// }

// export const dockerRegistry = new kx.Secret(
//     'docker-registry',
//     {
//         type: 'kubernetes.io/dockerconfigjson',
//         metadata: {
//             name: 'docker-registry-applications',
//             namespace: namespaces.applications,
//         },

//         data: {
//             // ".dockerconfigjson": JSON.stringify(dataRaw)
//             '.dockerconfigjson': toBase64(JSON.stringify(dataRaw)),
//         },
//     },
//     { provider: getResourceProvider('argocd-applications-parents', ENVIRONMENT) }
// );

// /*
// apiVersion: v1
// data:
//   .dockerconfigjson: eyJhdXRocyI6eyJnaHJjLmlvIjp7InVzZXJuYW1lIjoib3llIiwicGFzc3dvcmQiOiIxMjM0IiwiYXV0aCI6ImIzbGxPakV5TXpRPSJ9fX0=
// kind: Secret
// metadata:
//   creationTimestamp: null
//   name: rere
// type: kubernetes.io/dockerconfigjson

// */

import { providerArgoCDApplicationsParent } from './../../shared/createArgoApplication';
import { namespaceNames } from "./../../namespaces/util";
// import { getEnvironmentVariables } from './../../shared/validations';
import c from "chalk";
import path from "path";
import { getSecretsForApp } from "../../../scripts/secretsManagement/getSecretsForApp";
import { getArgocdParentApplicationsPath } from "../../shared/manifestsDirectory";
import { Environment } from "../../shared/types/own-types";
import sh from "shelljs";
import * as kx from "@pulumi/kubernetesx";
import { getEnvironmentVariables } from "../../shared/validations";

const { ENVIRONMENT } = getEnvironmentVariables();
const DOCKER_SERVER = "ghcr.io";
export const DOCKER_REGISTRY_KEY = "my-registry-key";

// export function createContainerRegistrySecret(environment: Environment): void {
//     const { username: DOCKER_USERNAME, password: DOCKER_PASSWORD } =
//         getSecretsForApp("argocd", environment);

//     const dir = path.join(
//         getArgocdParentApplicationsPath(environment),
//         "1-manifest"
//     );
//     const file = path.join(
//         dir,
//         // NOTE: has to be prefixed with the name "secret-". This is important for the CLI. I am considering using the file content instead but that might be more expensive operation
//         "secret-docker-registry.yaml"
//     );

//     if (!DOCKER_USERNAME || !DOCKER_PASSWORD) {
//         console.warn(
//             c.bgYellowBright("docker username nor password not provideed")
//         );
//         return;
//     }

//     sh.mkdir(dir);
//     sh.touch(file);

//     sh.exec(`
//   kubectl create secret docker-registry ${DOCKER_REGISTRY_KEY} --docker-server=${DOCKER_SERVER} \
//      --docker-username=${DOCKER_USERNAME} --docker-password=${DOCKER_PASSWORD} --namespace=${namespaceNames.applications} \
//      -o yaml --dry-run=client > ${file}`);
// }

interface DockerRawData {
    auths: {
        "ghrc.io": { username: string; password: string; auth: string };
    };
}

const { username: DOCKER_USERNAME, password: DOCKER_PASSWORD } = getSecretsForApp("argocd", ENVIRONMENT);
const dataRaw: DockerRawData = {
    auths: {
        "ghrc.io": {
            username: DOCKER_USERNAME,
            password: DOCKER_PASSWORD,
            auth: toBase64(`${DOCKER_USERNAME}:${DOCKER_PASSWORD}`)
        }
    }
}
function toBase64(text: string) {
    return Buffer.from(text).toString('base64')
}

export const dockerRegistry = new kx.Secret("docker-registry", {
    type: "kubernetes.io/dockerconfigjson",
    metadata: {
        name: "docker-registry-applications",
        namespace: namespaceNames.applications,
    },

    data: {
        // ".dockerconfigjson": JSON.stringify(dataRaw)
        ".dockerconfigjson": toBase64(JSON.stringify(dataRaw))
    },
}, { provider: providerArgoCDApplicationsParent });

/* 
apiVersion: v1
data:
  .dockerconfigjson: eyJhdXRocyI6eyJnaHJjLmlvIjp7InVzZXJuYW1lIjoib3llIiwicGFzc3dvcmQiOiIxMjM0IiwiYXV0aCI6ImIzbGxPakV5TXpRPSJ9fX0=
kind: Secret
metadata:
  creationTimestamp: null
  name: rere
type: kubernetes.io/dockerconfigjson

*/

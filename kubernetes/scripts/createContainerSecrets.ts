import sh from "shelljs";
import { getSecretsForApp } from "./secretsManagement/getSecretsForApp";

const DOCKER_SERVER = "ghcr.io";
const DOCKER_REGISTRY_KEY = "my-registry-key";
const { username: DOCKER_USERNAME, password: DOCKER_PASSWORD } = getSecretsForApp("argocd");

function deployContainerRegistrySecret() {
  sh.exec(
    `kubectl create secret docker-registry ${DOCKER_REGISTRY_KEY} --docker-server=${DOCKER_SERVER} \
     --docker-username=${DOCKER_USERNAME} --docker-password=${DOCKER_PASSWORD} \
     -o yaml --dry-run=client > //TODO-path-to-manifests-where-sealed-secrets will seal things up`
  );
}

// function deployContainerRegistrySecret() {
//   sh.exec(
//     `kubectl create secret docker-registry ${DOCKER_REGISTRY_KEY} --docker-server=${DOCKER_SERVER} \
//      --docker-username=${DOCKER_USERNAME} --docker-password=${DOCKER_PASSWORD} \
//      --docker-email=oyelowooyedayo@gmail.com -o yaml --dry-run=client`
//   );
// }

// import { getEnvironmentVariables } from './../../shared/validations';
import c from 'chalk';
import path from "path";
import { getSecretsForApp } from '../../../scripts/secretsManagement/getSecretsForApp';
import { getArgocdParentApplicationsPath } from '../../shared/manifestsDirectory';
import { Environment } from '../../shared/types/own-types';
import sh from "shelljs";

const DOCKER_SERVER = "ghcr.io";
const DOCKER_REGISTRY_KEY = "my-registry-key";

export function createContainerRegistrySecret(environment: Environment): void {
    const { username: DOCKER_USERNAME, password: DOCKER_PASSWORD } =
        getSecretsForApp("argocd", environment);

    const dir = path.join(
        getArgocdParentApplicationsPath(environment),
        "1-manifest",
    );
    const file = path.join(
        dir,
        // NOTE: has to be prefixed with the name "secret-". This is important for the CLI. I am considering using the file content instead but that might be more expensive operation
        "secret-docker-registry.yaml"
    );

    if (!DOCKER_USERNAME || !DOCKER_PASSWORD) {
        console.warn(c.bgYellowBright("docker username nor password not provideed"))
        return
    }

    sh.mkdir(dir)
    sh.touch(file);

    sh.exec(`
  kubectl create secret docker-registry ${DOCKER_REGISTRY_KEY} --docker-server=${DOCKER_SERVER} \
     --docker-username=${DOCKER_USERNAME} --docker-password=${DOCKER_PASSWORD} \
     -o yaml --dry-run=client > ${file}`);
}

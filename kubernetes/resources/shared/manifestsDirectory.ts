import { AppName, Environment } from "./types/own-types";
import path from "path";
import sh from "shelljs";

export const getMainBaseDir = () => {
  const mainBaseDir = path.join(__dirname, "..", "..");
  return mainBaseDir;
};

export const getManifestsBaseDir = () => {
  const MANIFESTS_DIR = path.join(getMainBaseDir(), "manifests");
  return MANIFESTS_DIR;
};

export const getUnsealedSecretsConfigFilesBaseDir = () => {
  return path.join(getMainBaseDir(), ".secrets");
};


export const getGeneratedEnvManifestsDir = (environment: Environment) => {
  const MANIFESTS_DIR = getManifestsBaseDir();
  return path.join(MANIFESTS_DIR, "generated", environment);
};

/**
 * e.g /manifests/generated/local/infrastructure/1-manifests
 *                               /infrastructure/1-crd
 *                               /infrastructure/sealed-secrets
 */
export const getPathToInfraToolDir = (toolName: string, environment: Environment) => {
  const MANIFESTS_BASE_DIR_FOR_ENV = getGeneratedEnvManifestsDir(environment);
  const dir = path.join(MANIFESTS_BASE_DIR_FOR_ENV, "infrastructure", toolName);
  // TODO: sh.mk`dir(dir);
  // sh.mk`dir(dir);
  return dir;
};

export const getRepoPathFromAbsolutePath = (absolutePath: string) => {
  const toolPath = absolutePath.split("/kubernetes/").at(-1);
  if (!toolPath) {
    throw new Error(`path not found`);
  }
  return path.join("kubernetes", toolPath);
};

export const argocdControllerName = "argocd";
export const getArgocdControllerDir = (environment: Environment) => {
  return getPathToInfraToolDir(argocdControllerName, environment);
};

export const certManagerControllerName = "cert-manager";
export const getCertManagerControllerDir = (environment: Environment) => {
  return getPathToInfraToolDir(certManagerControllerName, environment);
};

export const argocdApplicationsName = "argocd-applications";
export const argocdApplicationsDir = (environment: Environment) => {
  return getPathToInfraToolDir(argocdApplicationsName, environment);
};

export const linkerd2Name = "linkerd";
export const getLinkerd2Dir = (environment: Environment) => {
  return getPathToInfraToolDir(linkerd2Name, environment);
};

export const linkerdVizName = "linkerd-viz";
export const getLinkerdVizDir = (environment: Environment) => {
  return getPathToInfraToolDir(linkerdVizName, environment);
};

// TODO: Refactor to remove all these repititions
// type InfrastructureToolName = "cert-manager-controller" | 'sealed-secrets-controller'

// export const getInfraToolDir = (environment: Environment, infraTool: InfrastructureToolName) => {
//   return getPathToInfraToolDir(infraTool, environment);
// };
// infraName: InfrastructureToolName =  "cert-manager-controller"
// const kk = getInfraToolDir("production", "cert-manager-controller")

export const sealedSecretsControllerName = "sealed-secrets";
export const getSealedSecretsControllerDir = (environment: Environment) => {
  return getPathToInfraToolDir(sealedSecretsControllerName, environment);
};

export const ingressControllerName = "nginx-ingress";
export const getIngressControllerDir = (environment: Environment) => {
  return getPathToInfraToolDir(ingressControllerName, environment);
};

export const argocdApplicationName = "argocd-applications";
export const getArgocdApplicationsDir = (environment: Environment) => {
  return getPathToInfraToolDir(argocdApplicationName, environment);
};

export const getServiceDir = (appName: AppName, environment: Environment) => {
  return path.join(getGeneratedEnvManifestsDir(environment), "services", appName);
  // return `kubernetes/manifests/generated/${environment}/services/${appName}`;
};


export const getNamespacesNamesDir = (environment: Environment) => {
  return path.join(getGeneratedEnvManifestsDir(environment), "namespaces")

}

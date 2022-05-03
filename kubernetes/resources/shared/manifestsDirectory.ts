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
export const argoApplicationsNames = [
  "namespace-names",
  "sealed-secrets",
  "cert-manager",
  "nginx-ingress",
  "linkerd",
  "linkerd-viz",
  "argocd",
  "argocd-applications",
  "service"
] as const;

export type ArgoApplicationName = typeof argoApplicationsNames[number];
export type ResourceType = "infrastructure" | "services" | "namespaces" | "argo_applications_parents"

/**
 * e.g /manifests/generated/local/infrastructure/1-manifests
 *                               /infrastructure/1-crd
 *                               /infrastructure/sealed-secrets
 */
export const getPathToInfrastructureDir = (
  toolName: ArgoApplicationName,
  environment: Environment
) => {
  const MANIFESTS_BASE_DIR_FOR_ENV = getGeneratedEnvManifestsDir(environment);
  const dir = path.join(MANIFESTS_BASE_DIR_FOR_ENV, "infrastructure", toolName);
  // TODO: sh.mk`dir(dir);
  // sh.mk`dir(dir);
  return dir;
};

export const getPathToServicesDir = (appName: AppName | "argocd-applications", environment: Environment) => {
  return path.join(
    getGeneratedEnvManifestsDir(environment),
    "services",
    appName
  );
  // return `kubernetes/manifests/generated/${environment}/services/${appName}`;
};

export const getRepoPathFromAbsolutePath = (absolutePath: string) => {
  const toolPath = absolutePath.split("/kubernetes/").at(-1);
  if (!toolPath) {
    throw new Error(`path not found`);
  }
  return path.join("kubernetes", toolPath);
};


// export const getArgoAppSyncWaveAnnotation = (argoApplicationName: ArgoApplicationName) => {

//   return {
//     // https://argo-cd.readthedocs.io/en/stable/user-guide/sync-waves/#how-do-i-configure-waves
//     "argocd.argoproj.io/sync-wave": String(argoApplicationsNames.indexOf(argoApplicationName))
//   } as const
// }

export const argocdControllerName: ArgoApplicationName = "argocd";
export const getArgocdControllerDir = (environment: Environment) => {
  return getPathToInfrastructureDir(argocdControllerName, environment);
};

export const certManagerControllerName: ArgoApplicationName = "cert-manager";
export const getCertManagerControllerDir = (environment: Environment) => {
  return getPathToInfrastructureDir(certManagerControllerName, environment);
};



export const linkerd2Name: ArgoApplicationName = "linkerd";
export const getLinkerd2Dir = (environment: Environment) => {
  return getPathToInfrastructureDir(linkerd2Name, environment);
};

export const linkerdVizName: ArgoApplicationName = "linkerd-viz";
export const getLinkerdVizDir = (environment: Environment) => {
  return getPathToInfrastructureDir(linkerdVizName, environment);
};

// TODO: Refactor to remove all these repititions
// type InfrastructureToolName: InfrastructureName = "cert-manager-controller" | 'sealed-secrets-controller'

// export const getInfraToolDir = (environment: Environment, infraTool: InfrastructureToolName) => {
//   return getPathToInfraToolDir(infraTool, environment);
// };
// infraName: InfrastructureToolName =  "cert-manager-controller"
// const kk = getInfraToolDir("production", "cert-manager-controller")

export const sealedSecretsControllerName: ArgoApplicationName =
  "sealed-secrets";
export const getSealedSecretsControllerDir = (environment: Environment) => {
  return getPathToInfrastructureDir(sealedSecretsControllerName, environment);
};

export const ingressControllerName: ArgoApplicationName = "nginx-ingress";
export const getIngressControllerDir = (environment: Environment) => {
  return getPathToInfrastructureDir(ingressControllerName, environment);
};

export const argocdApplicationsName: ArgoApplicationName = "argocd-applications";
export const getArgocdInfraApplicationsDir = (environment: Environment) => {
  return getPathToInfrastructureDir(argocdApplicationsName, environment);
};


export const getArgocdServicesApplicationsDir = (environment: Environment) => {
  return getPathToServicesDir(argocdApplicationsName, environment);
};


export const getNamespacesNamesDir = (environment: Environment) => {
  return path.join(getGeneratedEnvManifestsDir(environment), "namespaces");
};

export const getNamespacesNamesArgoAppDir = (environment: Environment) => {
  return path.join(getNamespacesNamesDir(environment), argocdApplicationsName);
};

export const getArgoAppsParentsDir = (environment: Environment) => {
  return path.join(getGeneratedEnvManifestsDir(environment), "argo-applications-parents");
};

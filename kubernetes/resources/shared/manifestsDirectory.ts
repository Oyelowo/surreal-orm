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

/**
 * e.g /manifests/generated/local/1-manifests
 *                               /1-crd
 *                               /sealed-secrets
 */
export const getGeneratedEnvManifestsDir = (environment: Environment) => {
  const MANIFESTS_DIR = getManifestsBaseDir();
  return path.join(MANIFESTS_DIR, "generated", environment);
};

export const getPathToApplicationDirForEnv = (appName: AppName, environment: Environment) => {
  return path.join(getGeneratedEnvManifestsDir(environment), "applications", appName);
};

export const getPathToNonApplicationDir = (toolName: string, environment: Environment) => {
  const MANIFESTS_BASE_DIR_FOR_ENV = getGeneratedEnvManifestsDir(environment);
  const dir = path.join(MANIFESTS_BASE_DIR_FOR_ENV, toolName);
  sh.mkdir(dir);
  return dir;
};

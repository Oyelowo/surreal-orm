import { AppName, Environment } from "./types/own-types";
import * as k8s from "@pulumi/kubernetes";
import { getEnvironmentVariables } from "./validations";
import path from "path";

const { ENVIRONMENT } = getEnvironmentVariables();

// TODO: probably use path and __dirname modules?
export const getManifestsOutputDirectory = (environment: Environment) =>
  path.join("manifests", "generated", environment);

const MANIFESTS_BASE_DIR_FOR_ENV = getManifestsOutputDirectory(ENVIRONMENT);

// const APPLICATION_DIR = path.join(MANIFESTS_BASE_DIR_FOR_ENV, "applications");

export const getPathToApplicationDirForEnv = (appName: AppName, environment: Environment) => {
  return path.join(getManifestsOutputDirectory(environment), "applications", appName);
};

export const getPathToApplicationDir = (appName: AppName) => {
  return getPathToApplicationDirForEnv(appName, ENVIRONMENT);
};
// export const getPathToApplicationDir = (appName: AppName) => path.join(APPLICATION_DIR, appName);

export const getPathToNonApplicationDir = (toolName: string) => {
  return path.join(MANIFESTS_BASE_DIR_FOR_ENV, toolName);
};

export const getSealedSecretsDirForEnv = () => {
  return path.join(MANIFESTS_BASE_DIR_FOR_ENV, "sealed-secrets");
};

export const nameSpacesProvider = new k8s.Provider("render-namespaces", {
  renderYamlToDirectory: `${MANIFESTS_BASE_DIR_FOR_ENV}/namespaces`,
});

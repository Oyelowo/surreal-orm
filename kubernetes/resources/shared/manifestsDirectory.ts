import { AppName, Environment } from "./types/own-types";
import * as k8s from "@pulumi/kubernetes";
import { getEnvironmentVariables } from "./validations";
import path from "path";

const { ENVIRONMENT } = getEnvironmentVariables();

// TODO: probably use path and __dirname modules?
export const getManifestsOutputDirectory = (environment: Environment) =>
  path.join("manifests", "generated", environment);

const MANIFESTS_BASE_DIR_FOR_ENV = getManifestsOutputDirectory(ENVIRONMENT);
export const APPLICATION_DIR = path.join(MANIFESTS_BASE_DIR_FOR_ENV, "applications");

export const getPathToApplicationDir = (appName: AppName) => path.join(APPLICATION_DIR, appName);
export const getPathToNonApplicationDir = (toolName: string) =>
  path.join(MANIFESTS_BASE_DIR_FOR_ENV, toolName);

export const nameSpacesProvider = new k8s.Provider("render-namespaces", {
  renderYamlToDirectory: `${MANIFESTS_BASE_DIR_FOR_ENV}/namespaces`,
});

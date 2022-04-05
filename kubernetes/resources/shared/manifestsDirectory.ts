import { AppName, Environment } from "./types/own-types";
import path from "path";

// const { ENVIRONMENT } = getEnvironmentVariables();

// TODO: probably use path and __dirname modules?
export const getManifestsOutputDirectory = (environment: Environment) =>
  path.join("manifests", "generated", environment);

export const getPathToApplicationDirForEnv = (appName: AppName, environment: Environment) => {
  return path.join(getManifestsOutputDirectory(environment), "applications", appName);
};

export const getPathToNonApplicationDir = (toolName: string, environment: Environment) => {
  const MANIFESTS_BASE_DIR_FOR_ENV = getManifestsOutputDirectory(environment);
  return path.join(MANIFESTS_BASE_DIR_FOR_ENV, toolName);
};

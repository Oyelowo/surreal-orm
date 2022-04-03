import * as k8s from "@pulumi/kubernetes";
import { getPathToApplicationDir } from "../shared/manifestsDirectory";

export const reactWebDirectoryPath = getPathToApplicationDir("react-web");

export const reactWebProvider = new k8s.Provider(reactWebDirectoryPath, {
  renderYamlToDirectory: reactWebDirectoryPath,
});

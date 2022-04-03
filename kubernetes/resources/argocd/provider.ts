import * as k8s from "@pulumi/kubernetes";
import { getPathToApplicationDir } from "../shared/manifestsDirectory";

export const argocdDirectoryPath = getPathToApplicationDir("argocd");

export const argocdProvider = new k8s.Provider(argocdDirectoryPath, {
  renderYamlToDirectory: argocdDirectoryPath,
});

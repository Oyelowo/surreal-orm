import * as k8s from "@pulumi/kubernetes";
import { getPathToApplicationDir } from "../shared/manifestsDirectory";

export const ingressControllerDirectoryPath = getPathToApplicationDir("ingress-controller");

export const ingressControllerProvider = new k8s.Provider(ingressControllerDirectoryPath, {
  renderYamlToDirectory: ingressControllerDirectoryPath,
});

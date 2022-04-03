import * as k8s from "@pulumi/kubernetes";
import { getPathToApplicationDir } from "../shared/manifestsDirectory";

export const sealedSecretsDirectoryPath = getPathToApplicationDir("sealed-secrets");

export const sealedSecretsProvider = new k8s.Provider(sealedSecretsDirectoryPath, {
  renderYamlToDirectory: sealedSecretsDirectoryPath,
});

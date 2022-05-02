import { getLinkerdBootstrapDir } from "../../shared/manifestsDirectory";
import { getEnvironmentVariables } from "../../shared/validations";
import * as k8s from "@pulumi/kubernetes";

export const linkerdBootstrapDir = getLinkerdBootstrapDir(
    getEnvironmentVariables().ENVIRONMENT
);

export const linkerdBootstrapProvider = new k8s.Provider(linkerdBootstrapDir, {
    renderYamlToDirectory: linkerdBootstrapDir,
});

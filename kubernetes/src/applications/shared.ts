import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";

// Instantiate a Kubernetes Provider and specify the render directory.
export const provider = new k8s.Provider("render-yaml", {
  renderYamlToDirectory: "rendered",
});

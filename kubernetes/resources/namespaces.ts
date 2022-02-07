import { provider } from "./cluster";
import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
import { Namespace } from "@pulumi/kubernetes/core/v1";
// import * as eks from "@pulumi/eks";

export const namespace = new k8s.core.v1.Namespace("development", undefined, {
  provider,
});

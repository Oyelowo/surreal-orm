import * as k8s from '@pulumi/kubernetes';
import * as kx from '@pulumi/kubernetesx';

import { provider } from '../shared/cluster';
import { devNamespaceName } from '../shared/namespaces';
import { Settings } from '../shared/types';
import { reactWebSettings } from './settings';

const { envVars, kubeConfig, metadata } = reactWebSettings;

const resourceName = kubeConfig.resourceName;

// Create a Kubernetes ConfigMap.
export const reactWebConfigMap = new kx.ConfigMap(
  `${resourceName}-configmap`,
  {
    metadata,
    data: { config: "very important data" },
  },
  { provider }
);

// Create a Kubernetes Secret.
export const reactWebSecret = new kx.Secret(
  `${resourceName}-secret`,
  {
    metadata,
    stringData: {
      password: "very-weak-password",
    },
  },
  { provider }
);

// Define a Pod.
export const reactWebPodBuilder = new kx.PodBuilder({
  initContainers: [],
  containers: [
    {
      env: {
        CONFIG: reactWebConfigMap.asEnvValue("config"),
        PASSWORD: reactWebSecret.asEnvValue("password"),
        ...envVars,
      },
      image: kubeConfig.image,
      ports: { http: Number(envVars.APP_PORT) },
      volumeMounts: [],
      resources: {
        limits: {
          memory: kubeConfig.limitMemory,
          cpu: kubeConfig.limitCpu,
        },
        requests: {
          memory: kubeConfig.requestMemory,
          cpu: kubeConfig.requestCpu,
        },
      },
    },
  ],
});

// Create a Kubernetes Deployment.
export const reactWebDeployment = new kx.Deployment(
  `${resourceName}-deployment`,
  {
    metadata,
    spec: reactWebPodBuilder.asDeploymentSpec({ replicas: 2 }),
  },
  { provider }
);

// // Create a Kubernetes Service.
export const reactWebService = reactWebDeployment.createService({
  type: kx.types.ServiceType.ClusterIP,
});

console.log("rerekrekrek", reactWebService.urn);

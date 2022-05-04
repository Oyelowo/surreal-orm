import { AppName, Environment } from "./types/own-types";
import path from "path";
import sh from "shelljs";
import * as k8s from "@pulumi/kubernetes";
// TODO:  Unify all the resourceType/resourceName path utils into a singular function e.g

export const getMainBaseDir = () => {
  const mainBaseDir = path.join(__dirname, "..", "..");
  return mainBaseDir;
};

export const getManifestsBaseDir = () => {
  const MANIFESTS_DIR = path.join(getMainBaseDir(), "manifests");
  return MANIFESTS_DIR;
};

export const getUnsealedSecretsConfigFilesBaseDir = () => {
  return path.join(getMainBaseDir(), ".secrets");
};

export const getGeneratedEnvManifestsDir = (environment: Environment) => {
  const MANIFESTS_DIR = getManifestsBaseDir();
  return path.join(MANIFESTS_DIR, "generated", environment);
};

export const argoApplicationsNames = [
  "namespace-names",
  "sealed-secrets",
  "cert-manager",
  "nginx-ingress",
  "linkerd",
  "linkerd-viz",
  "argocd",
  "argocd-applications",
  // "service",
] as const;

export type ArgoApplicationName = typeof argoApplicationsNames[number];
export type ResourceType =
  | "infrastructure"
  | "services"
  | "namespaces"
  | "argo_applications_parents";
export type ResourceName = ArgoApplicationName | AppName;

export const getPathToResoucrcesDir = (
  appName: ResourceName,
  resourceType: ResourceType,
  environment: Environment
) => {
  return path.join(
    getGeneratedEnvManifestsDir(environment),
    resourceType,
    appName
  );
  // return `kubernetes/manifests/generated/${environment}/services/${appName}`;
};

/**
 * e.g /manifests/generated/local/infrastructure/1-manifests
 *                               /infrastructure/1-crd
 *                               /infrastructure/sealed-secrets
 */

export const getRepoPathFromAbsolutePath = (absolutePath: string) => {
  const toolPath = absolutePath.split("/kubernetes/").at(-1);
  if (!toolPath) {
    throw new Error(`path not found`);
  }
  return path.join("kubernetes", toolPath);
};

type GetPathToResourceProps = {
  resourceType: ResourceType;
  // resourceName: Omit<ArgoApplicationName | AppName, "service">
  resourceName: ResourceName;
  environment: Environment;
};

export const getPathToResource = (props: GetPathToResourceProps): string => {
  const resourcePath = path.join(
    getGeneratedEnvManifestsDir(props.environment),
    props.resourceType,
    props.resourceName
  );
  return resourcePath;
};
import { v4 as uuid } from "uuid";

export const createProvider = (props: GetPathToResourceProps) => {
  return new k8s.Provider(
    `${props.resourceType}-${props.resourceName}-${uuid()}`,
    {
      renderYamlToDirectory: getPathToResource(props),
    }
  );
};

type ResourcePropertiesReturn = {
  pathAbsolute: string;
  provider: k8s.Provider;
  pathRelative: string;
  resourceName: ResourceName;
  resourceType: ResourceType;
};


function getResourceProperties<T>(
  resourceName: ResourceName,
  environment: Environment,
  getResourceProps: (resourceName: ResourceType) => T
): T {
  switch (resourceName) {
    case "react-web":
    case "graphql-mongo":
    case "graphql-postgres":
    case "grpc-mongo": {
      return getResourceProps("services");
    }

    case "argocd":
    case "cert-manager":
    case "linkerd":
    case "linkerd":
    case "sealed-secrets":
    case "linkerd-viz":
    case "namespace-names":
    case "nginx-ingress": {
      return getResourceProps("infrastructure");
    }

    case "argocd-applications": {
      return getResourceProps("argo_applications_parents");
    }
  }
  return assertUnreachable(resourceName);
}

export function assertUnreachable(x: never): never {
  throw new Error("Didn't expect to get here");
}

// export function getResourceAbsolutePath(
//   resourceName: ResourceName,
//   environment: Environment
// ): string {
//   return getResourceProperties(resourceName, environment).pathAbsolute;
// }


// export function getResourceRelativePath(
export function getResourceProvider(
  resourceName: ResourceName,
  environment: Environment
): k8s.Provider {
  return getResourceProperties(resourceName, environment, (resourceType) => createProvider({
    resourceName,
    resourceType,
    environment
  })
  );
}


export function getResourceAbsolutePath(
  resourceName: ResourceName,
  environment: Environment
): string {
  return getResourceProperties(resourceName, environment, (resourceType) => getPathToResource({
    resourceName,
    resourceType,
    environment
  })
  );
}

export function getResourceRelativePath(
  resourceName: ResourceName,
  environment: Environment
): string {
  const pathAbsolute = getResourceAbsolutePath(resourceName, environment)
  // provider: createProvider(props),
  return getRepoPathFromAbsolutePath(pathAbsolute)

}

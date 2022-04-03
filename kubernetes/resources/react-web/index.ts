import { reactWebProvider, reactWebDirectoryPath } from "./provider";
import { getPathToApplicationDir } from "../shared/manifestsDirectory";
import { ServiceDeployment } from "../shared/deployment";
import { reactWebSettings } from "./settings";
import * as k8s from "@pulumi/kubernetes";
import { createArgocdApplication } from "../shared/createArgoApplicaiton";

export const reactWebArgocdApplication = createArgocdApplication({
  metadata: {
    name: reactWebSettings.metadata.name,
    namespace: reactWebSettings.metadata.namespace,
  },
  provider: reactWebProvider,
  pathToAppManifests: reactWebDirectoryPath,
});

export const reactWeb = new ServiceDeployment("react-web", reactWebSettings, {
  provider: reactWebProvider,
});

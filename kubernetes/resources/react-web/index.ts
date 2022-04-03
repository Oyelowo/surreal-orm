import { getPathToApplicationDir } from "../shared/manifestsDirectory";
import { ServiceDeployment } from "../shared/deployment";
import { reactWebSettings } from "./settings";
import * as k8s from "@pulumi/kubernetes";
import { createArgocdApplication } from "../shared/createArgoApplicaiton";

const appDir = getPathToApplicationDir("react-web");

export const reactWebDirectory = new k8s.Provider("render-react", {
  renderYamlToDirectory: appDir,
});

export const reactWebArgocdApplication = createArgocdApplication({
  metadata: {
    name: reactWebSettings.metadata.name,
    namespace: reactWebSettings.metadata.namespace,
  },
  provider: reactWebDirectory,
  pathToAppManifests: appDir,
});

export const reactWeb = new ServiceDeployment("react-web", reactWebSettings, {
  provider: reactWebDirectory,
});

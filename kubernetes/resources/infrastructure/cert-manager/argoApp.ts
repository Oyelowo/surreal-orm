import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { createArgocdApplication, certManagerControllerName, namespaceNames } from "../../shared";
import { certManagerControllerDir } from "./certManager";


export const certManagerApplication = createArgocdApplication({
  metadata: {
    name: certManagerControllerName,
    namespace: namespaceNames.default,
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(certManagerControllerDir),
});

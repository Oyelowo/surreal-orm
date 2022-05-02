import { namespaceNames } from "../../namespaces/util";
import { createArgocdApplication } from "../../shared/createArgoApplication";
import { certManagerControllerName, getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";

import { certManagerControllerDir } from "./certManager";


export const certManagerApplication = createArgocdApplication({
  metadata: {
    name: certManagerControllerName,
    namespace: namespaceNames.certManager,
  },
  pathToAppManifests: getRepoPathFromAbsolutePath(certManagerControllerDir),
});

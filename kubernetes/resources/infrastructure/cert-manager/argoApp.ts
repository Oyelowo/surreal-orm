import { getRepoPathFromAbsolutePath } from "../../shared/manifestsDirectory";
import { createArgocdApplication, certManagerControllerName, namespaceNames } from "../../shared";
import { certManagerControllerDir } from "./certManager";

type Metadata = {
  name: string;
  namespace: string;
};

const metadataCertManager: Metadata = {
  name: certManagerControllerName,
  namespace: namespaceNames.default,
};

export const certManagerApplication = createArgocdApplication({
  metadata: { ...metadataCertManager },
  pathToAppManifests: getRepoPathFromAbsolutePath(certManagerControllerDir),
});

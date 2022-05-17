#!/usr/bin / env ts - node

import { helmChartsInfo } from "../../resources/shared/helmChartInfo";
import { namespaceNames } from "../../resources/namespaces/util";
import {
  getArgocdParentApplicationsPath,
  getResourceAbsolutePath,
  ResourceName,
} from "../../resources/shared/manifestsDirectory";
import sh from "shelljs";
/* 
TODO: ADD INSTRUCTION ON HOW THIS WORKS
*/
// TODO: Maybe setup base argocd app that deploys other argocd apps?
import { ArgumentTypes } from "../../../typescript/apps/web/utils/typescript";
// TODO: Allow the selections of applications to regenerate secret for. This should be done with inquirer prompt.
// This would read the name of the app = name of deployment in manifests to determine the sealed secrets  to regenerate and override
import path from "path";
import { Environment } from "../../resources/shared/types/own-types";
import { setupPlainSecretTSFiles } from "../secretsManagement/setupSecrets";
import { generateManifests } from "./generateManifests";
import { getImageTagsFromDir } from "./getImageTagsFromDir";
import { generateAllSealedSecrets } from "./generateAllSealedSecrets";

export async function bootstrapCluster(environment: Environment) {
  const imageTags = await getImageTagsFromDir();

  await generateManifests({
    environment,
    imageTags,
  });

  setupPlainSecretTSFiles();

  /*
       This requires the above step with initial cluster setup making sure that the sealed secret controller is
       running in the cluster */

  // # Apply namespace first
  applyResourceManifests("namespace-names", environment);

  // # Apply setups with sealed secret controller
  applyResourceManifests("sealed-secrets", environment);

  const sealedSecretsName: ResourceName = "sealed-secrets";
  // # Wait for bitnami sealed secrets controller to be in running phase so that we can use it to encrypt secrets
  sh.exec(
    `kubectl rollout status deployment/${sealedSecretsName} -n=${namespaceNames.kubeSystem}`
  );

  // # Apply setups with cert-manager controller
  applyResourceManifests("cert-manager", environment);

  // # Wait for cert-manager and cert-manager-trust controllers to be in running phase so that we can use it to encrypt secrets
  const { certManager, certManagerTrust } = helmChartsInfo.jetspackRepo;
  sh.exec(
    `kubectl rollout status deployment/${certManager.chart} -n=${namespaceNames.certManager}`
  );
  sh.exec(
    `kubectl rollout status deployment/${certManagerTrust.chart} -n=${namespaceNames.certManager}`
  );

  // # Apply setups with linkerd controller
  applyResourceManifests("linkerd", environment);
  applyResourceManifests("linkerd-viz", environment);

  await generateAllSealedSecrets({
    environment,
  });

  // TODO: could conditionally check the installation of argocd also cos it may not be necessary for local dev
  applyResourceManifests("argocd", environment);
  // TODO: Split bootstrap process from restart from update
  sh.exec(`kubectl -n argocd rollout restart deployment argocd-argo-cd-server`);

  // TODO: Only apply this in non prod environment
  sh.exec(
    `kubectl apply -R -f ${getArgocdParentApplicationsPath(environment)}`
  );
}

function applyResourceManifests(
  resourceName: ResourceName,
  environment: Environment
) {
  const resourceDir = getResourceAbsolutePath(resourceName, environment);
  const applyManifests = (dir: string) =>
    sh.exec(`kubectl apply -R -f  ${path.join(resourceDir, dir)}`);
  ["sealed-secrets", "0-crd", "1-manifest"].forEach(applyManifests);

  // sh.exec(`kubectl apply -R -f  ${resourceDir}/sealed-secrets`);
  // sh.exec(`kubectl apply -R -f  ${resourceDir}/0-crd`);
  // sh.exec(`kubectl apply -R -f  ${resourceDir}/1-manifest`);
}

import * as k8s from "@pulumi/kubernetes";
import { getArgocdControllerDir } from "../../shared/manifestsDirectory";
import { namespaceNames } from "../../shared/namespaces";
import { ArgocdHelmValuesArgo } from "../../shared/types/helm-charts/argocdHelmValuesArgo";
import { DeepPartial } from "../../shared/types/own-types";
import { getEnvironmentVariables } from "../../shared/validations";
import bcrypt from "bcrypt"

const { ENVIRONMENT } = getEnvironmentVariables();
const argocdControllerDir = getArgocdControllerDir(ENVIRONMENT);



export const argocdControllerProvider = new k8s.Provider(argocdControllerDir, {
    renderYamlToDirectory: argocdControllerDir,
});


const saltRounds = 10;
const myPlaintextPassword = 'oyelowo';
const hash = bcrypt.hashSync(myPlaintextPassword, saltRounds);
const argocdValues: DeepPartial<ArgocdHelmValuesArgo> = {
    fullnameOverride: "argocd",
    server: {

    },
    configs: {
        secret: {
            // createSecret: false,
            argocdServerAdminPassword: hash,
        }
    }
    ,
    dex: {
        enabled: false

    },
    redis: {
        enabled: true
    },
    notifications: {
        enabled: true,
        secret: {
            create: true,
            items: {
                "name": "ererer"
            }
        }
    }
    // redis: {

    // }
};

export const argocdHelm = new k8s.helm.v3.Chart(
    "argocd",
    {
        chart: "argo-cd",
        fetchOpts: {
            repo: "https://argoproj.github.io/argo-helm",
        },
        version: "4.5.3",
        values: argocdValues,
        namespace: namespaceNames.argocd,
        // namespace: devNamespaceName,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: argocdControllerProvider }
);


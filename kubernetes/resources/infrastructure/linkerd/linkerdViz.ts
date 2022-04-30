import { LinkerdVizHelmValues } from './../../shared/types/helm-charts/linkerdVizHelmValues';
import { helmChartsInfo } from './../../shared/helmChartInfo';
import * as k8s from "@pulumi/kubernetes";

import { linkerd2Name } from "../../shared/manifestsDirectory";
import { namespaceNames } from "../../shared/namespaces";
import { DeepPartial } from "../../shared/types/own-types";
import { linkerdProvider } from './linkerd';


const values: DeepPartial<LinkerdVizHelmValues> = {

};


const { repo, linkerdViz: { chart, version } } = helmChartsInfo.linkerdRepo;
export const linkerdViz = new k8s.helm.v3.Chart(
    linkerd2Name,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values,
        namespace: namespaceNames.linkerd,
        // namespace: devNamespaceName,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: linkerdProvider }
    // { provider }
);

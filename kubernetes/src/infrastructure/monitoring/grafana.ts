import { IGrafanaGrafana } from "../../../generatedHelmChartsTsTypes/grafanaGrafana.js";
import * as k8s from "@pulumi/kubernetes";
import { DeepPartial, namespaces } from "../../types/ownTypes.js";
import { monitoringProvider } from "./settings.js";
import { helmChartsInfo } from "../../shared/helmChartInfo.js";

const grafanaValues: DeepPartial<IGrafanaGrafana> = {};

const {
	repo,
	charts: { grafana: { chart, version } },
} = helmChartsInfo.grafana;

export const grafanaHelm = new k8s.helm.v3.Chart(
	chart,
	{
		chart,
		fetchOpts: {
			repo,
		},
		version,
		values: grafanaValues,
		namespace: namespaces.monitoring,
		skipAwait: false,
	},
	{ provider: monitoringProvider },
);

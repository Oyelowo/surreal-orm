import { IVeleroVmwareTanzu } from "../../../generatedHelmChartsTsTypes/veleroVmwareTanzu.js";
import * as k8s from "@pulumi/kubernetes";
import { DeepPartial, namespaces } from "../../types/ownTypes.js";
import { veleroProvider } from "./settings.js";
import { helmChartsInfo } from "../../shared/helmChartInfo.js";

const veleroValues: DeepPartial<IVeleroVmwareTanzu> = {
	backupsEnabled: true,
	configuration: {
		backupStorageLocation: {},
	},
};

const {
	repo,
	charts: { velero: { chart, version } },
} = helmChartsInfo.vmwareTanzu;

export const veleroHelm = new k8s.helm.v3.Chart(
	chart,
	{
		chart,
		fetchOpts: {
			repo,
		},
		version,
		values: veleroValues,
		namespace: namespaces.velero,
		skipAwait: false,
	},
	{ provider: veleroProvider },
);

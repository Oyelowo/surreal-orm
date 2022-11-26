import { ILonghornLonghorn } from "../../../generatedHelmChartsTsTypes/longhornLonghorn.js";
import * as k8s from "@pulumi/kubernetes";

import { helmChartsInfo } from "../../shared/helmChartInfo.js";
import { DeepPartial, namespaces } from "../../types/ownTypes.js";
import { longhornOperatorProvider } from "./settings.js";

const longhornOperatValues: DeepPartial<ILonghornLonghorn> = {};

// `http://${name}.${namespace}:${port}`;
const {
	repo,
	charts: { longhorn: { chart, version, externalCrds } },
} = helmChartsInfo.longhorn;

export const longhornNodeRequirements = new k8s.yaml.ConfigGroup(
	"longhorn-node-requirements",
	{
		files: [
			// ICSI and NFS are required to be installed on the node where longhorn
			// will be installed cos it needs them to provision the storage
			`https://raw.githubusercontent.com/longhorn/longhorn/${version}/deploy/prerequisite/longhorn-iscsi-installation.yaml`,
			`https://raw.githubusercontent.com/longhorn/longhorn/${version}/deploy/prerequisite/longhorn-nfs-installation.yaml`,
		],
	},
	{ provider: longhornOperatorProvider },
);

export const longhornSystem = new k8s.helm.v3.Chart(
	chart,
	{
		chart,
		fetchOpts: {
			repo,
		},
		version,
		values: longhornOperatValues,
		namespace: namespaces.longhornSystem,
		// By default Release resource will wait till all created resources
		// are available. Set this to true to skip waiting on resources being
		// available.
		skipAwait: false,
	},
	{ provider: longhornOperatorProvider },
);

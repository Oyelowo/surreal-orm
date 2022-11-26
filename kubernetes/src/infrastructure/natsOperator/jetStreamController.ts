import { INackNats } from "../../../generatedHelmChartsTsTypes/nackNats.js";
import * as k8s from "@pulumi/kubernetes";
import { helmChartsInfo } from "../../shared/helmChartInfo.js";
import { DeepPartial, namespaces } from "../../types/ownTypes.js";
import { natsOperatorProvider } from "./settings.js";

const natsJetStreamControllerValues: DeepPartial<INackNats> = {
	jetstream: {
		enabled: true,
	},
};

// `http://${name}.${namespace}:${port}`;
const {
	repo,
	charts: { nack: { chart, version, externalCrds } },
} = helmChartsInfo.nats;

// CRDs
// nats operator helm chart does not include the crds. That's why we're handling it separately
export const natsCrds = new k8s.yaml.ConfigGroup(
	chart,
	{
		files: externalCrds,
	},
	{ provider: natsOperatorProvider },
);

export const natsJetStreamController = new k8s.helm.v3.Chart(
	chart,
	{
		chart,
		fetchOpts: {
			repo,
		},
		version,
		values: natsJetStreamControllerValues,
		namespace: namespaces.natsOperator,
		// By default Release resource will wait till all created resources
		// are available. Set this to true to skip waiting on resources being
		// available.
		skipAwait: false,
	},
	{ provider: natsOperatorProvider },
);

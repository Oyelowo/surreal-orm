import { IGiteaGitea } from "../../../generatedHelmChartsTsTypes/giteaGitea.js";
import * as k8s from "@pulumi/kubernetes";
import { DeepPartial, namespaces } from "../../types/ownTypes.js";
import { argoEventProvider } from "./settings.js";
import { helmChartsInfo } from "../../shared/helmChartInfo.js";

const argoEventValues: DeepPartial<IGiteaGitea> = {};

const {
	repo,
	charts: { gitea: { chart, version } },
} = helmChartsInfo.gitea;

export const argoEventHelm = new k8s.helm.v3.Chart(
	chart,
	{
		chart,
		fetchOpts: {
			repo,
		},
		version,
		values: argoEventValues,
		namespace: namespaces.argoEvent,
		skipAwait: false,
	},
	{ provider: argoEventProvider },
);

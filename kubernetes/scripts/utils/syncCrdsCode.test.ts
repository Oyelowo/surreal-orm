import { sanitizePulumiTypeDefinitions } from "./syncCrdsCode";

describe("syncCrdsCode", () => {
	test("Can sanitize badly formatted Pulumi typescript definition", () => {
		// Removes hyphen from interface and fielf type identifiers and puts keys with hyphen in quote
		const data = `export interface TidbClu-sterStatusArgs {
            auto-scaler?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusAuto-ScalerArgs>;
            cluste?: pulumi.Input<string>;
            co-ndi-ti-ons-?: pulumi.Input<pulumi.Input<inputs.pingcap.v1alpha1.TidbCluster-Status-ConditionsArgs>[]>;
            p-d: pulumi.Input<inputs.pin-gcap.v1alpha1.TidbClusterStatusPdArgs>;
            pump?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusPumpArgs>;
            ticdc?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusTicdcArgs>;
            tidb?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusTidbArgs>;
            tiflash?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusTiflashArgs>;
            tikv?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusTikvArgs>;
        }

        export interface TidbClusterSta-tusAuto-ScalerArgs {
            name: pulumi.Input<string>;
            namespace: pulumi.Input<string>;
        }`;
		const sanitized = sanitizePulumiTypeDefinitions({ data });

		expect(sanitized).toBe(`export interface TidbClusterStatusArgs {
            "auto-scaler"?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusAutoScalerArgs>;
            cluste?: pulumi.Input<string>;
            "co-ndi-ti-ons-"?: pulumi.Input<pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusConditionsArgs>[]>;
            "p-d": pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusPdArgs>;
            pump?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusPumpArgs>;
            ticdc?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusTicdcArgs>;
            tidb?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusTidbArgs>;
            tiflash?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusTiflashArgs>;
            tikv?: pulumi.Input<inputs.pingcap.v1alpha1.TidbClusterStatusTikvArgs>;
        }

        export interface TidbClusterStatusAutoScalerArgs {
            name: pulumi.Input<string>;
            namespace: pulumi.Input<string>;
        }`);
	});
});

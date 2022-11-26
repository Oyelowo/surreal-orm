import * as linode from "@pulumi/linode";
import * as pulumi from "@pulumi/pulumi";

type LinodeRegionId =
	| "ap-west"
	| "ca-central"
	| "ap-southeast"
	| "us-central"
	| "us-west"
	| "us-southeast"
	| "us-east"
	| "eu-west"
	| "ap-south"
	| "eu-central"
	| "ap-northeast";

export const LINODE_REGION_ID: LinodeRegionId = "eu-central";
const region = pulumi.output(
	linode.getRegion({
		id: LINODE_REGION_ID,
	}),
);
// const REGION = "er"

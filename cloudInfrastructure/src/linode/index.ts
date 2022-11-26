export * from "./domain.js";
export * from "./lkeCluster.js";
export * from "./nodeBalancer.js";
// Typical cloud resources I might want to deploy for my kubernetes cluster
// linode.NodeBalancer
// linode.LkeCluster
// linode.Domain
// linode.Firewall
// linode.SshKey
// linode.User

// Miscellanneous
// linode.Volume
// linode.ObjectStorageObject

// Reference
// const my_cluster = pulumi.output(linode.getLkeCluster({
//     id: 69_005,
// }));
// pulumi config set --secret dbPassword S3cr37
// const config = new pulumi.Config();
// export const region = pulumi.output(linode.getRegion({
//     id: LINODE_REGION_ID,
// }));
// const name = config.require("name");
// const dbPassword = config.requireSecret("dbPassword");
// console.log("ddd", dbPassword)

// console.log(`Password: ${config.require("dbPassword")}`);

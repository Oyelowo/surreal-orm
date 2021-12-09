import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";

const providerKube = new k8s.Provider("docker-pro", {
  renderYamlToDirectory: "generated",
});

// const appLabels = { app: "nginx" };
// const deployment = new k8s.apps.v1.Deployment(
//   "nginx",
//   {
//     spec: {
//       selector: { matchLabels: appLabels },
//       replicas: 1,
//       template: {
//         metadata: { labels: appLabels },
//         spec: {
//           containers: [
//             {
//               name: "nginx",
//               image: "nginx",
//               resources: {
//                 limits: { memory: "128Mi", cpu: "500m" },
//                 requests: {},
//               },
//             },
//           ],
//         },
//       },
//     },
//   },
//   { provider: providerKube }
// );
// export const name = deployment.metadata.name;

import * as docker from "@pulumi/docker";

const provider = new docker.Provider("docker-pro", {});

const image = new docker.RemoteImage(
  "ubuntu",
  {
    name: "ubuntu:precise",
  },
  { provider }
);

const container = new docker.Container(
  "ubuntu",
  {
    image: image.latest,
  },
  { provider }
);

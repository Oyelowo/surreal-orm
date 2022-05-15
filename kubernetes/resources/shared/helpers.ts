import { AppConfigs } from "./types/own-types";
import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";

// TODO: Load values.yaml from helm bitnami repo
// Convert yaml to json
// Convert json to TS

// Similar to https://jsonformatter.org/yaml-to-typescript

export function getFQDNFromSettings(config: AppConfigs<any, any, any>) {
  const { namespace, name } = config.metadata;
  const host = config.envVars.APP_PORT;
  return `${name}.${namespace}:${host}`;
}

interface ServiceProps {
  serviceName: string;
  deployment: kx.Deployment;
  args: kx.types.ServiceSpec;
}

// NOT USED FOR NOW. I went with directly patching the package instead. Keep for future purpose/reference
export function generateService({ serviceName, deployment, args = {} }: ServiceProps): kx.Service {
  const serviceSpec = pulumi
    .all([deployment.spec.template.spec.containers, args])
    .apply(([containers, args]) => {
      // TODO: handle merging ports from args
      const ports: Record<string, number> = {};
      containers.forEach((container) => {
        if (container.ports) {
          container.ports.forEach((port) => {
            ports[port.name] = port.containerPort;
          });
        }
      });
      return {
        ...args,
        ports: args.ports || ports,
        selector: deployment.spec.selector.matchLabels,
        // TODO: probably need to unwrap args.type in case it's a computed value
        type: args && (args.type as string),
      };
    });

  return new kx.Service(
    serviceName,
    {
      metadata: { namespace: deployment.metadata.namespace, name: serviceName },
      spec: serviceSpec,
    },
    { /* ...deployment.opts ,*/ parent: deployment }
  );
}



// // deployment: kx.Deployment, args: types.ServiceSpec = {}
// function createService2({serviceName, deployment, args= {}}: ServiceProps) {
//   const serviceSpec = pulumi
//     .all([deployment.spec.template.spec.containers, args])
//     .apply(([containers, args]) => {
//       // TODO: handle merging ports from args
//       const ports = {};
//       containers.forEach((container) => {
//         if (container.ports) {
//           container.ports.forEach((port) => {
//             ports[port.name] = port.containerPort;
//           });
//         }
//       });
//       return Object.assign(Object.assign({}, args), {
//         ports: args.ports || ports,
//         selector: deployment.spec.selector.matchLabels,
//         // TODO: probably need to unwrap args.type in case it's a computed value
//         type: args && args.type,
//       });
//     });

//   return new kx.Service(
//     serviceName,
//     {
//       metadata: { namespace: deployment.metadata.namespace },
//       spec: serviceSpec,
//     },
//     Object.assign(Object.assign({}, deployment.opts), { parent: deployment })
//   );
// }

// // class WrappedDeployment extends kx.Deployment {

// //     createService(args?: types.ServiceSpec): kx.Service {
// //           const serviceSpec = pulumi
// //             .all([this.spec.template.spec.containers, args])
// //             .apply(([containers, args]) => {
// //               // TODO: handle merging ports from args
// //               const ports = {};
// //               containers.forEach((container) => {
// //                 if (container.ports) {
// //                   container.ports.forEach((port) => {
// //                     ports[port.name] = port.containerPort;
// //                   });
// //                 }
// //               });
// //               return Object.assign(Object.assign({}, args), {
// //                 ports: args.ports || ports,
// //                 selector: this.spec.selector.matchLabels,
// //                 // TODO: probably need to unwrap args.type in case it's a computed value
// //                 type: args && args.type,
// //               });
// //             });

// //           return new kx.Service(
// //             serviceName,
// //             {
// //               metadata: { namespace: this.metadata.namespace },
// //               spec: serviceSpec,
// //             },
// //             Object.assign(Object.assign({}, this.opts), {
// //               parent: this,
// //             })
// //           );
// //     }
// // }

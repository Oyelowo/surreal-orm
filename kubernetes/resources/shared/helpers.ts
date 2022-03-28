import { AppName } from "./types/own-types";

import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";
import { RawJSON } from "@pulumi/pulumi/automation";
import secretsJonFile from "./secrets-dont-push.json";

import * as z from "zod";

const secretsSchema = z.object({
  "graphql-mongo": z.object({
    MONGODB_USERNAME: z.string().nonempty(),
    MONGODB_PASSWORD: z.string().nonempty(),
    REDIS_USERNAME: z.string().nonempty(),
    REDIS_PASSWORD: z.string().nonempty(),
  }),
  "grpc-mongo": z.object({
    MONGODB_USERNAME: z.string().nonempty(),
    MONGODB_PASSWORD: z.string().nonempty(),
  }),
  "graphql-postgres": z.object({
    POSTGRES_USERNAME: z.string().nonempty(),
    POSTGRES_PASSWORD: z.string().nonempty(),
  }),
  "react-web": z.object({}),
});

export const secretsJsonToBase64 = z
  .function()
  .args(secretsSchema)
  .returns(secretsSchema)
  .strictImplement((json) => {
    const parsedSecrets = secretsSchema.parse(json);
    const entries = Object.entries(parsedSecrets).map(([appName, appEnvVars]) => {
      const appObjectToArraysEncodedValues = Object.entries(appEnvVars).map(
        ([secretName, secretValue]) => [
          secretName,
          Buffer.from(secretValue).toString("base64"),
        ]
      );

      return [appName, Object.fromEntries(appObjectToArraysEncodedValues)];
      // Turn it back into an object
    });

    return Object.fromEntries(entries);
  });
// function getSecretForApp(appName: keyof z.infer<typeof secretsSchema>) {
export function getSecretForApp<T extends AppName>(
  appName: AppName
): z.infer<typeof secretsSchema>[T] {
  return secretsJsonToBase64(secretsJonFile)[appName] as z.infer<
    typeof secretsSchema
  >[T];
}

// const base64data = btoa("someText");
// export function secretsJsonToBase642(json: RawJSON) {
//   const parsedSecrets = secretsSchema.parse(json);

//   const entries = Object.entries(parsedSecrets).map(([key, value]) => [
//     key,
//     btoa(value),
//   ]);
//   return Object.fromEntries(entries);
// }
interface ServiceProps {
  serviceName: string;
  deployment: kx.Deployment;
  args: kx.types.ServiceSpec;
}

// NOT USED FOR NOW. I went with directly patching the package instead. Keep for future purpose/reference
export function generateService({
  serviceName,
  deployment,
  args = {},
}: ServiceProps): kx.Service {
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

import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";
import z from "zod";
import { AppConfigs, NamespaceOfApps, ServiceName } from "../types/ownTypes.js";

export function getFQDNFromSettings<
	N extends ServiceName,
	NS extends NamespaceOfApps,
>(config: AppConfigs<N, NS, Record<string, string>>) {
	const { namespace, name } = config.metadata;
	const host = z.string().parse(config.envVars?.APP_PORT);
	return `${name}.${namespace}:${host}`;
}

interface ServiceProps {
	serviceFileName: string;
	deployment: kx.Deployment;
	args: kx.types.ServiceSpec;
}

// NOT USED FOR NOW. I went with directly patching the kx package instead. Keep for future purpose/reference
export function generateService({
	serviceFileName,
	deployment,
	args = {},
}: ServiceProps): kx.Service {
	const serviceSpec = pulumi
		.all([deployment.spec.template.spec.containers, args])
		.apply(([containers, args]) => {
			// CONSIDER: handle merging ports from args
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
				// CONSIDER: probably need to unwrap args.type in case it's a computed value
				type: args && (args.type as string),
			};
		});

	return new kx.Service(
		serviceFileName,
		{
			metadata: deployment.metadata,
			spec: serviceSpec,
		},
		{ parent: deployment },
	);
}

export function toBase64(text: string): string {
	return Buffer.from(text).toString("base64");
}

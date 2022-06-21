import * as kx from '@pulumi/kubernetesx';
import * as pulumi from '@pulumi/pulumi';
import { AppConfigs } from '../types/own-types';

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

// NOT USED FOR NOW. I went with directly patching the kx package instead. Keep for future purpose/reference
export function generateService({ serviceName, deployment, args = {} }: ServiceProps): kx.Service {
    const serviceSpec = pulumi.all([deployment.spec.template.spec.containers, args]).apply(([containers, args]) => {
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
        { parent: deployment }
    );
}

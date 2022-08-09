import { Environment } from '../../types/ownTypes.js';
import getPort, { portNumbers } from 'get-port';

// export const DOMAIN_NAME_SUB_ARGOCD = `argocd.${HOST_INGRESS}`;
// export const DOMAIN_NAME_SUB_LINKERD_VIZ = 'linkerd.oyedev.com';

// Incrementally find next available port from 8080 for exposing ingress,
// otherwise fall back to a random port
export const INGRESS_EXTERNAL_PORT_LOCAL = await getPort({ port: portNumbers(8080, 8200) });

export const ingressControllerPorts = {
    http: 80, // Maps to INGRESS_EXTERNAL_PORT_LOCAL with k3d in the make file
    https: 443,
} as const;

type Subdomain = 'argocd' | 'linkerd' | 'grafana' | 'prometheus';

// For syncing all local ETC hosts on macbook
const subs: Subdomain[] = ['argocd', 'grafana', 'linkerd', 'prometheus'];
const baseHostLocal = 'oyelowo.local';
export const ingressHostsLocal = ['localhost', baseHostLocal, ...subs.map((d) => `${d}.${baseHostLocal}`)];

const BASE_INGRESS_HOST = 'oyelowo.dev';
type IngressBaseHost = `localhost:${number}` | typeof BASE_INGRESS_HOST;
type IngressHost = `${Subdomain}.${IngressBaseHost}` | IngressBaseHost;

const ingressBaseHosts: Record<Environment, IngressBaseHost> = {
    // For local host, we add a port, otherwise leave out
    local: `localhost:${INGRESS_EXTERNAL_PORT_LOCAL}`,
    development: BASE_INGRESS_HOST,
    staging: BASE_INGRESS_HOST,
    production: BASE_INGRESS_HOST,
};

type Prop = {
    environment: Environment;
    subDomain?: Subdomain;
};

export function getIngressUrlHost({ environment, subDomain }: Prop): IngressHost {
    const baseHost = ingressBaseHosts[environment];
    return !!subDomain ? `${subDomain}.${baseHost}` : baseHost;
}

type IngressDomain = `http${'' | 's'}://${IngressHost}`;
export function getIngressUrl({ environment, subDomain }: Prop): IngressDomain {
    const protocol = `http${environment === 'local' ? '' : 's'}://` as const;
    const host = getIngressUrlHost({
        environment,
        subDomain,
    });
    return `${protocol}${host}`;
}

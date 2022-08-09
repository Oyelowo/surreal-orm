import { Environment } from '../../types/ownTypes.js';
import { DOMAIN_NAME_BASE } from './constant.js';
import getPort, { portNumbers } from 'get-port';

// type DomainBase = 'oyelowo.local' | typeof DOMAIN_NAME_BASE;
type DomainBase = `localhost` | typeof DOMAIN_NAME_BASE;

// Incrementally find next available port from 8080 for exposing ingress,
// otherwise fall back to a random port
export const INGRESS_EXTERNAL_PORT_LOCAL = await getPort({ port: portNumbers(8080, 8200) });

type Domain = `http://${DomainBase}:${typeof INGRESS_EXTERNAL_PORT_LOCAL}` | `https://${DomainBase}`;
interface Hosts {
    base: DomainBase;
    // api: `api.${DomainBase}`;
    domain: Domain;
    apiUrl?: `${Domain}/api`;
    port?: typeof INGRESS_EXTERNAL_PORT_LOCAL;
}

type Configs = Record<Environment, Hosts>;
const api = (base: DomainBase) => `${base}/api` as const;

export const ingressControllerPorts = {
    http: 80, // Maps to INGRESS_EXTERNAL_PORT_LOCAL with k3d in the make file
    https: 443,
} as const;

export const hosts: Configs = {
    local: {
        // base: `localhost:${INGRESS_EXTERNAL_PORT_LOCAL}`,
        base: `localhost`,
        domain: `http://localhost:${INGRESS_EXTERNAL_PORT_LOCAL}`,
        // api: 'api.oyelowo.local',
        // apiUrl: `http://localhost:${INGRESS_EXTERNAL_PORT_LOCAL}/api`,
        port: INGRESS_EXTERNAL_PORT_LOCAL,
    },
    development: {
        base: DOMAIN_NAME_BASE,
        domain: `https://${DOMAIN_NAME_BASE}`,
        // api: api(DOMAIN_NAME_BASE),
    },
    staging: {
        base: DOMAIN_NAME_BASE,
        domain: `https://${DOMAIN_NAME_BASE}`,
        // api: api(DOMAIN_NAME_BASE),
    },
    production: {
        base: DOMAIN_NAME_BASE,
        domain: `https://${DOMAIN_NAME_BASE}`,
        // api: api(DOMAIN_NAME_BASE),
    },
};

export function getBaseUrl(environment: Environment) {
    const host = hosts[environment];
    // For local host, we add a port, otherwise leave out
    return environment === 'local' ? `http://${host.base}:${host.port}` : host.base;
}

import { Environment } from '../../types/own-types';
import { DOMAIN_NAME_BASE } from './constant';

// type DomainBase = 'localhost' | typeof DOMAIN_NAME_BASE;
type DomainBase = 'oyelowo.local' | typeof DOMAIN_NAME_BASE;

interface Hosts {
    base: DomainBase;
    api: `api.${DomainBase}`;
    port?: number;
}

type Configs = Record<Environment, Hosts>;
const api = (base: DomainBase) => `api.${base}` as const;

export const hosts: Configs = {
    local: {
        base: 'oyelowo.local',
        api: 'api.oyelowo.local',
        port: 8080
    },
    development: {
        base: DOMAIN_NAME_BASE,
        api: api(DOMAIN_NAME_BASE),
    },
    staging: {
        base: DOMAIN_NAME_BASE,
        api: api(DOMAIN_NAME_BASE),
    },
    production: {
        base: DOMAIN_NAME_BASE,
        api: api(DOMAIN_NAME_BASE),
    },
};


export function getBaseUrl(environment: Environment) {
    const host = hosts[environment];
    // For local host, we add a port, otherwise leave out
    return `${host.base}${host.port ? `:${host.port}` : ''}`
}
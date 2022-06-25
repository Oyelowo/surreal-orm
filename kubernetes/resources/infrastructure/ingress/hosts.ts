import { Environment } from "../../types/own-types";
import { DOMAIN_NAME_BASE } from "./constant";

// type DomainBase = 'localhost' | typeof DOMAIN_NAME_BASE;
type DomainBase = 'oyelowo.local' | typeof DOMAIN_NAME_BASE;

interface Hosts {
    base: DomainBase;
    api: `api.${DomainBase}`;
}

type Configs = Record<Environment, Hosts>;
const api = (base: DomainBase) => `api.${base}` as const;

export const hosts: Configs = {
    local: {
        base: 'oyelowo.local',
        api: 'api.oyelowo.local',
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
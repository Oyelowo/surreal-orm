import { Environment } from './../../types/own-types';
import * as k8s from '@pulumi/kubernetes';
import { namespaceNames } from '../../namespaces/util';
import { graphqlMongoSettings } from '../../services/graphql-mongo/settings';
import { reactWebSettings } from '../../services/react-web/settings';
import { NginxConfiguration } from '../../types/nginxConfigurations';
import { getEnvironmentVariables } from '../../shared/validations';
import { CLUSTER_ISSUER_NAME } from '../cert-manager';
import { DOMAIN_NAME_BASE } from './constant';
import { nginxIngressProvider } from './settings';

const { ENVIRONMENT } = getEnvironmentVariables();

type IngressClassName = 'nginx' | 'traefik';
export const INGRESS_CLASSNAME_NGINX: IngressClassName = 'nginx';
const SECRET_NAME_NGINX = 'nginx-ingress-tls';

const name = 'oyelowo-ingress';

type DomainBase = 'localhost' | typeof DOMAIN_NAME_BASE;

interface Hosts {
    base: DomainBase;
    api: `api.${DomainBase}`;
}

type Configs = Record<Environment, Hosts>;
const api = (base: DomainBase) => `api.${base}` as const;

const hosts: Configs = {
    local: {
        base: 'localhost',
        api: 'api.localhost',
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

const getHosts = (environemnt: Environment) => Object.values(hosts[environemnt]) as string[];

type CertManagerAnnotations = {
    // NOTE: Make sure you specify the right one, if using cluster-issuer, user cluster-issuer annotations, otherwise, use mere issuer
    // which is namespaced
    'cert-manager.io/cluster-issuer': typeof CLUSTER_ISSUER_NAME;
    'cert-manager.io/issuer': string; // We don't yet have an issuer. We are still using cluster issuer
};

type IngressAnnotations = NginxConfiguration & CertManagerAnnotations;
export const annotations: Partial<IngressAnnotations> = {
    'nginx.ingress.kubernetes.io/ssl-redirect': 'false',
    'nginx.ingress.kubernetes.io/use-regex': 'true',
    'cert-manager.io/cluster-issuer': CLUSTER_ISSUER_NAME,
};
export const appIngress = new k8s.networking.v1.Ingress(
    name,
    {
        metadata: {
            name,
            namespace: namespaceNames.applications,
            annotations: annotations as any,
        },
        spec: {
            ingressClassName: INGRESS_CLASSNAME_NGINX,
            tls: [
                {
                    hosts: getHosts(ENVIRONMENT),
                    secretName: SECRET_NAME_NGINX,
                },
            ],
            rules: [
                {
                    host: hosts[ENVIRONMENT].base,
                    http: {
                        paths: [
                            {
                                pathType: 'Prefix',
                                // path: "/?(.*)",
                                path: '/',
                                backend: {
                                    service: {
                                        name: reactWebSettings.metadata.name,
                                        port: { number: Number(reactWebSettings.envVars.APP_PORT) },
                                    },
                                },
                            },
                        ],
                    },
                },
                {
                    host: hosts[ENVIRONMENT].api,
                    http: {
                        paths: [
                            {
                                pathType: 'Prefix',
                                path: '/',
                                backend: {
                                    service: {
                                        name: graphqlMongoSettings.metadata.name,
                                        port: { number: Number(graphqlMongoSettings.envVars.APP_PORT) },
                                    },
                                },
                            },
                        ],
                    },
                },
            ],
        },
    },
    { provider: nginxIngressProvider }
);

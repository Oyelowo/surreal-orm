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

type Configs = Record<Environment, { hosts: string[]; host: string }>;
const configs: Configs = {
    local: {
        hosts: ['localhost'],
        host: 'localhost',
    },
    development: {
        hosts: [DOMAIN_NAME_BASE],
        host: DOMAIN_NAME_BASE,
    },
    staging: {
        hosts: [DOMAIN_NAME_BASE],
        host: DOMAIN_NAME_BASE,
    },
    production: {
        hosts: [DOMAIN_NAME_BASE],
        host: DOMAIN_NAME_BASE,
    },
};

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
                    hosts: configs[ENVIRONMENT].hosts,
                    secretName: SECRET_NAME_NGINX,
                },
            ],
            rules: [
                {
                    host: configs[ENVIRONMENT].host,
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
                            {
                                pathType: 'Prefix',
                                path: '/graphql',
                                backend: {
                                    service: {
                                        name: graphqlMongoSettings.metadata.name,
                                        port: {
                                            number: Number(graphqlMongoSettings.envVars.APP_PORT),
                                        },
                                    },
                                },
                            },
                            // {
                            //   pathType: "Prefix",
                            //   path: "/graphql",
                            //   backend: {
                            //     service: {
                            //       name: graphqlPostgresSettings.metadata.name,
                            //       port: {
                            //         number: Number(graphqlPostgresSettings.envVars.APP_PORT),
                            //       },
                            //     },
                            //   },
                            // },
                        ],
                    },
                },
                // {
                //   // Replace this with your own domain!
                //   host: "myserviceb.foo.org",
                //   http: {
                //     paths: [
                //       {
                //         pathType: "Prefix",
                //         path: "/",
                //         backend: {
                //           service: {
                //             name: graphqlPostgresSettings.resourceName,
                //             port: { number: Number(graphqlPostgresEnvVars.APP_PORT) },
                //           },
                //         },
                //       },
                //     ],
                //   },
                // },
            ],
        },
    },
    { provider: nginxIngressProvider }
);

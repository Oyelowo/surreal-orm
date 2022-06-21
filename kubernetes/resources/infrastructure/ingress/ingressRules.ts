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

const appBase = 'oyelowo';
// // Next, expose the app using an Ingress.

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
    `${appBase}-ingress`,
    {
        metadata: {
            name: `${appBase}-ingress`,
            namespace: namespaceNames.applications,
            annotations: annotations as any,
        },
        spec: {
            ingressClassName: INGRESS_CLASSNAME_NGINX,
            tls: [
                {
                    hosts: [DOMAIN_NAME_BASE],
                    secretName: SECRET_NAME_NGINX,
                },
            ],
            rules: [
                {
                    // Replace this with your own domain!
                    // host: "myservicea.foo.org",
                    // TODO: Change to proper domain name for prod and other environments in case of necessity
                    host: ENVIRONMENT === 'local' ? 'localhost' : DOMAIN_NAME_BASE,
                    // host: ENVIRONMENT === "local" ? "localhost" : "172.104.255.25",
                    // host: ENVIRONMENT === "local" ? "oyelowo.dev" : "oyelowo.dev",
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

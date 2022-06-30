import { ResourceName } from './../../resources/types/own-types';

// This function does nothing. It just helps with typing
type AppEnvVars = Record<ResourceName, Record<string, string>>;
const makeType = <T extends AppEnvVars>(o: T) => o

// NOTE: I initially was encoding the secrets in base64 but it turns out
// that bitnami sealed secrets not only handles encryption but base64 encoding of the
// secrets before encrypting them
export const secretsSample = makeType({
    'graphql-mongo': {
        MONGODB_USERNAME: 'example',
        MONGODB_PASSWORD: 'example',
        MONGODB_ROOT_USERNAME: 'example',
        MONGODB_ROOT_PASSWORD: 'example',
        REDIS_USERNAME: 'example',
        REDIS_PASSWORD: 'example',
        GITHUB_CLIENT_ID: 'example',
        GITHUB_CLIENT_SECRET: 'example',
        GOOGLE_CLIENT_ID: 'example',
        GOOGLE_CLIENT_SECRET: 'example',
    },
    'grpc-mongo': {
        MONGODB_USERNAME: 'example',
        MONGODB_PASSWORD: 'example',
        MONGODB_ROOT_USERNAME: 'example',
        MONGODB_ROOT_PASSWORD: 'example',
    },
    'graphql-postgres': {
        POSTGRES_USERNAME: 'example',
        POSTGRES_PASSWORD: 'example',
    },
    'react-web': {},
    argocd: {
        ADMIN_PASSWORD: 'example',
        type: 'git',
        url: 'https://github.com/Oyelowo/modern-distributed-app-template',
        username: 'Oyelowo',
        password: 'my-password-or-personal-access-token',
    },
    'argocd-applications-children-infrastructure': {},
    'argocd-applications-children-services': {},
    'argocd-applications-parents': {},
    'cert-manager': {},
    linkerd: {},
    'linkerd-viz': {},
    'namespaces': {},
    'nginx-ingress': {},
    'sealed-secrets': {},
});


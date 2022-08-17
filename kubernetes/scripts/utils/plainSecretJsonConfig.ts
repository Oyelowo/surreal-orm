import { ENVIRONMENTS_ALL } from '../utils/shared.js';
import path from 'node:path';
import sh from 'shelljs';
import { Environment, EnvironmentVariableKey, EnvVarAll, ResourceName } from '../../src/resources/types/ownTypes.js';
import { z } from 'zod';
import * as R from 'ramda';
import _ from 'lodash';
import { getPlainSecretsConfigFilesBaseDir } from '../../src/resources/shared/directoriesManager.js';
import prettier from 'prettier'
// prettier.format()
// Note: If these starts growing too much, we can separate
// each apps schema and merge them all
export const getSecretsSchema = ({
    allowEmptyValues,
    environment,
}: {
    allowEmptyValues: boolean;
    environment: Environment;
}) => {
    const isLocal = environment === 'local';
    const getDefault =
        (defaultValue: string) =>
            (v: string): string =>
                isLocal && _.isEmpty(v) ? defaultValue : v;
    const stringNoDefault = z.string().min(allowEmptyValues ? 0 : 1);
    const string = stringNoDefault
        // We want to populate the values for local environment
        // for the first time but not for other environments
        // So that the dev can properly see which secrets have not been added
        // it's okay to make mistake in local
        .transform(getDefault('example'));

    const secretsSample = z.object({
        SERVICE__GRAPHQL_MONGO__MONGODB_USERNAMEs: string,
        SERVICE__GRAPHQL_MONGO__MONGODB_PASSWORD: string,
        SERVICE__GRAPHQL_MONGO__MONGODB_ROOT_USERNAME: string,
        SERVICE__GRAPHQL_MONGO__MONGODB_ROOT_PASSWORD: string,
        SERVICE__GRAPHQL_MONGO__REDIS_USERNAME: string,
        SERVICE__GRAPHQL_MONGO__REDIS_PASSWORD: string,
        SERVICE__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_ID: string,
        SERVICE__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_SECRET: string,
        SERVICE__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_ID: string,
        SERVICE__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_SECRET: string,

        SERVICE__GRPC_MONGO__MONGODB_USERNAME: string,
        SERVICE__GRPC_MONGO__MONGODB_PASSWORD: string,
        SERVICE__GRPC_MONGO__MONGODB_ROOT_USERNAME: string,
        SERVICE__GRPC_MONGO__MONGODB_ROOT_PASSWORD: string,

        SERVICE__GRAPHQL_POSTGRES__POSTGRES_USERNAME: string,
        SERVICE__GRAPHQL_POSTGRES__POSTGRES_PASSWORD: string,

        SERVICE__REACT_WEB__: string,

        INFRASTRUCTURE__ARGOCD__ADMIN_PASSWORD: string,
        INFRASTRUCTURE__ARGOCD__TYPE: stringNoDefault.transform(getDefault('git')),
        INFRASTRUCTURE__ARGOCD__URL: stringNoDefault.transform(getDefault('https://github.com/Oyelowo/modern-distributed-app-template')),
        INFRASTRUCTURE__ARGOCD__USERNAME: stringNoDefault.transform(getDefault('Oyelowo')),
        INFRASTRUCTURE__ARGOCD__PASSWORD: string,

        INFRASTRUCTURE__ARGOCD_APPLICATIONS_CHILDREN_INFRASTRUCTURE__: string,
        INFRASTRUCTURE__ARGOCD_APPLICATIONS_CHILDREN_SERVICES__: string,
        INFRASTRUCTURE__ARGOCD_APPLICATIONS_PARENTS__: string,

        INFRASTRUCTURE__CERT_MANAGER__: string,
        INFRASTRUCTURE__LINKERD__: string,
        INFRASTRUCTURE__LINKERD_VIZ__PASSWORD: string,
        INFRASTRUCTURE__NAMESPACES__: string,

        INFRASTRUCTURE__NGINX_INGRESS__: string,

        INFRASTRUCTURE__SEALED_SECRETS__: string,
    });

    return secretsSample;
};

export type TSecretJson = z.infer<ReturnType<typeof getSecretsSchema>>;

const checkType = <T extends EnvVarAll<string>>(obj: T): T => obj;
const kk: EnvVarAll<string> = {

}
// <ResourceCategory>__<ResourceName>__<A secret key>
const secretsSample1: TSecretJson = checkType({
    SERVICE__GRAPHQL_MONGO__MONGODB_USERNAME: "",
    SERVICE__GRAPHQL_MONGO__MONGODB_PASSWORD: "",
    SERVICE__GRAPHQL_MONGO__MONGODB_ROOT_USERNAME: "",
    SERVICE__GRAPHQL_MONGO__MONGODB_ROOT_PASSWORD: "",
    SERVICE__GRAPHQL_MONGO__REDIS_USERNAME: "",
    SERVICE__GRAPHQL_MONGO__REDIS_PASSWORD: "",
    SERVICE__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_ID: "",
    SERVICE__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_SECRET: "",
    SERVICE__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_ID: "",
    SERVICE__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_SECRET: "",

    SERVICE__GRPC_MONGO__MONGODB_USERNAME: "",
    SERVICE__GRPC_MONGO__MONGODB_PASSWORD: "",
    SERVICE__GRPC_MONGO__MONGODB_ROOT_USERNAME: "",
    SERVICE__GRPC_MONGO__MONGODB_ROOT_PASSWORD: "",

    SERVICE__GRAPHQL_POSTGRES__POSTGRES_USERNAME: "",
    SERVICE__GRAPHQL_POSTGRES__POSTGRES_PASSWORD: "",

    SERVICE__REACT_WEB__: "",

    INFRASTRUCTURE__ARGOCD__ADMIN_PASSWORD: "",
    INFRASTRUCTURE__ARGOCD__TYPE: "",
    INFRASTRUCTURE__ARGOCD__URL: "",
    INFRASTRUCTURE__ARGOCD__USERNAME: "",
    INFRASTRUCTURE__ARGOCD__PASSWORD: "",

    INFRASTRUCTURE__ARGOCD_APPLICATIONS_CHILDREN_INFRASTRUCTURE__: "",
    INFRASTRUCTURE__ARGOCD_APPLICATIONS_CHILDREN_SERVICES__: "",
    INFRASTRUCTURE__ARGOCD_APPLICATIONS_PARENTS__: "",

    INFRASTRUCTURE__CERT_MANAGER__: "",
    INFRASTRUCTURE__LINKERD__: "",
    INFRASTRUCTURE__LINKERD_VIZ__PASSWORD: "",
    INFRASTRUCTURE__NAMESPACES__: "",

    INFRASTRUCTURE__NGINX_INGRESS__: "",

    INFRASTRUCTURE__SEALED_SECRETS__: "",
} as const);

const secretsSample: TSecretJson = checkType({
    'graphql-mongo': {
        MONGODB_USERNAME: '',
        MONGODB_PASSWORD: '',
        MONGODB_ROOT_USERNAME: '',
        MONGODB_ROOT_PASSWORD: '',
        REDIS_USERNAME: '',
        REDIS_PASSWORD: '',
        OAUTH_GITHUB_CLIENT_ID: '',
        OAUTH_GITHUB_CLIENT_SECRET: '',
        OAUTH_GOOGLE_CLIENT_ID: '',
        OAUTH_GOOGLE_CLIENT_SECRET: '',
    },
    'grpc-mongo': {
        MONGODB_USERNAME: '',
        MONGODB_PASSWORD: '',
        MONGODB_ROOT_USERNAME: '',
        MONGODB_ROOT_PASSWORD: '',
    },
    'graphql-postgres': { POSTGRES_USERNAME: '', POSTGRES_PASSWORD: '' },
    'react-web': {},
    argocd: {
        ADMIN_PASSWORD: '',
        type: '',
        url: '',
        username: '',
        password: '',
    },
    'argocd-applications-children-infrastructure': {},
    'argocd-applications-children-services': {},
    'argocd-applications-parents': {},
    'cert-manager': {},
    linkerd: {},
    'linkerd-viz': { PASSWORD: '' },
    namespaces: {},
    'nginx-ingress': {},
    'sealed-secrets': {},
} as const);

const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();

export class PlainSecretJsonConfig<App extends ResourceName> {
    constructor(private resourceName: App, private environment: Environment) { }

    getSecrets = (): TSecretJson[App] => {
        // PlainSecretJsonConfig.syncAll();

        const secretsSchema = getSecretsSchema({
            allowEmptyValues: false,
            environment: this.environment,
        });

        const allSecretsJson = PlainSecretJsonConfig.#getSecretJsonObject(this.environment);
        return secretsSchema.strict().parse(allSecretsJson)[this.resourceName];
    };

    static resetValues = (environment: Environment): void => {
        sh.echo(`Empting secret JSON config for ${environment}`);
        sh.mkdir('-p', PLAIN_SECRETS_CONFIGS_DIR);
        const envPath = this.#getSecretPath(environment);

        sh.exec(`echo '${JSON.stringify(secretsSample)}' > ${envPath}`);
        sh.exec(`npx prettier --write ${envPath}`);
    };

    static syncAll = (): void => {
        ENVIRONMENTS_ALL.forEach((environment) => {
            sh.echo(`Syncing Secret JSON config for ${environment}`);
            sh.mkdir('-p', PLAIN_SECRETS_CONFIGS_DIR);

            const envPath = this.#getSecretPath(environment);
            const existingEnvSecret = this.#getSecretJsonObject(environment) ?? {};

            if (_.isEmpty(existingEnvSecret)) sh.touch(envPath);

            // Allows us to only get valid keys out, so we can parse the merged secrets out.
            const secretsSchema = getSecretsSchema({ allowEmptyValues: true, environment });
            // Parse the object to filter out stale keys in existing local secret configs
            // This also persists the values of existing secrets
            const mergedObject = secretsSchema.parse(R.mergeDeepLeft(existingEnvSecret, secretsSample));

            sh.exec(`echo '${JSON.stringify(mergedObject)}' > ${envPath}`);
            sh.exec(`npx prettier --write ${envPath}`);
        });
    };

    // static #getSecretJsonObject = (environment: Environment): object | undefined => {
    //     const envPath = PlainSecretJsonConfig.#getSecretPath(environment);

    //     const existingEnvSecret = this.#parseJson<object>(sh.exec(`cat ${envPath}`, { silent: true }).stdout.trim());
    //     return existingEnvSecret;
    // };

    static #getSecretPath = (environment: Environment): string => {
        return path.join(PLAIN_SECRETS_CONFIGS_DIR, `${environment}.json`);
    };


}

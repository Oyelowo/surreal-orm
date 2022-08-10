import { ENVIRONMENTS_ALL } from '../utils/shared.js';
import path from 'node:path';
import sh from 'shelljs';
import { Environment, ResourceName } from '../../src/resources/types/ownTypes.js';
import { z } from 'zod';
import * as R from 'ramda';
import _ from 'lodash';
import { getPlainSecretsConfigFilesBaseDir } from '../../src/resources/shared/directoriesManager.js';

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
    const string = z
        .string()
        .min(allowEmptyValues ? 0 : 1)
        // We want to populate the values for local environment
        // for the first time but not for other environments
        // So that the dev can properly see which secrets have not been added
        // it's okay to make mistake in local
        .default(() => (isLocal ? 'example' : ''));

    const secretsSample = z.object({
        'graphql-mongo': z.object({
            MONGODB_USERNAME: string,
            MONGODB_PASSWORD: string,
            MONGODB_ROOT_USERNAME: string,
            MONGODB_ROOT_PASSWORD: string,
            REDIS_USERNAME: string,
            REDIS_PASSWORD: string,
            OAUTH_GITHUB_CLIENT_ID: string,
            OAUTH_GITHUB_CLIENT_SECRET: string,
            OAUTH_GOOGLE_CLIENT_ID: string,
            OAUTH_GOOGLE_CLIENT_SECRET: string,
        }),
        'grpc-mongo': z.object({
            MONGODB_USERNAME: string,
            MONGODB_PASSWORD: string,
            MONGODB_ROOT_USERNAME: string,
            MONGODB_ROOT_PASSWORD: string,
        }),
        'graphql-postgres': z.object({
            POSTGRES_USERNAME: string,
            POSTGRES_PASSWORD: string,
        }),
        'react-web': z.object({}),
        argocd: z.object({
            ADMIN_PASSWORD: string,
            type: string.transform(() => 'git'),
            url: string.transform((_item) => 'https://github.com/Oyelowo/modern-distributed-app-template'),
            username: string.transform(() => 'Oyelowo'),
            password: string,
        }),
        'argocd-applications-children-infrastructure': z.object({}),
        'argocd-applications-children-services': z.object({}),
        'argocd-applications-parents': z.object({}),
        'cert-manager': z.object({}),
        linkerd: z.object({}),
        'linkerd-viz': z.object({
            PASSWORD: string,
        }),
        namespaces: z.object({}),
        'nginx-ingress': z.object({}),
        'sealed-secrets': z.object({}),
    });

    return secretsSample;
};

export type TSecretJson = z.infer<ReturnType<typeof getSecretsSchema>>;

const checkType = <T extends Record<ResourceName, unknown>>(obj: T): T => obj;

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
        OAUTH_GOOGLE_CLIENT_SECRET: ''
    },
    'grpc-mongo': {
        MONGODB_USERNAME: '',
        MONGODB_PASSWORD: '',
        MONGODB_ROOT_USERNAME: '',
        MONGODB_ROOT_PASSWORD: ''
    },
    'graphql-postgres': { POSTGRES_USERNAME: '', POSTGRES_PASSWORD: '' },
    'react-web': {},
    argocd: {
        ADMIN_PASSWORD: '',
        type: '',
        url: '',
        username: '',
        password: ''
    },
    'argocd-applications-children-infrastructure': {},
    'argocd-applications-children-services': {},
    'argocd-applications-parents': {},
    'cert-manager': {},
    linkerd: {},
    'linkerd-viz': { PASSWORD: '' },
    namespaces: {},
    'nginx-ingress': {},
    'sealed-secrets': {}
} as const);



const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();

export class PlainSecretJsonConfig<App extends ResourceName> {
    constructor(private resourceName: App, private environment: Environment) { }

    getSecrets = (): TSecretJson[App] => {
        PlainSecretJsonConfig.syncAll();

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

    static #getSecretJsonObject = (environment: Environment): object | undefined => {
        const envPath = PlainSecretJsonConfig.#getSecretPath(environment);

        const existingEnvSecret = this.#parseJson<object>(sh.exec(`cat ${envPath}`, { silent: true }).stdout.trim());
        return existingEnvSecret;
    };

    static #getSecretPath = (environment: Environment): string => {
        return path.join(PLAIN_SECRETS_CONFIGS_DIR, `${environment}.json`);
    };

    static #parseJson = <T>(json: string): T | undefined => {
        try {
            return JSON.parse(json) as T;
        } catch {
            return undefined;
        }
    }
}

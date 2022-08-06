import { ENVIRONMENTS_ALL } from '../utils/shared.js';
import path from 'node:path';
import sh from 'shelljs';
import { Environment, ResourceName } from '../../resources/types/own-types.js';
import { generateMock } from '@anatine/zod-mock';
import { z } from 'zod';
import * as R from 'ramda';
import _ from 'lodash';
import { getPlainSecretsConfigFilesBaseDir } from '../../resources/shared/manifestsDirectory.js';

// Note: If these starts growing too much, we can separate
// each apps schema and merge them all
const getSecretsSample = ({
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

export type TSecretJson = z.infer<ReturnType<typeof getSecretsSample>>;

function emptyObjectValues(object: any) {
    Object.keys(object).forEach((k) => {
        if (object[k] && typeof object[k] === 'object') {
            return emptyObjectValues(object[k]);
        }
        object[k] = '';
    });
}

const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();

export class PlainSecretJsonConfig<App extends ResourceName> {
    constructor(private resourceName: App, private environment: Environment) {}

    getSecrets = (): TSecretJson[App] => {
        PlainSecretJsonConfig.syncAll();

        const secretsSchema = PlainSecretJsonConfig.getSecretsSchema({
            allowEmptyValues: false,
            environment: this.environment,
        });

        const allSecretsJson = PlainSecretJsonConfig.#getSecretJsonObject(this.environment);
        return secretsSchema.strict().parse(allSecretsJson)[this.resourceName];
    };

    static emptyValues = (environment: Environment): void => {
        sh.exec(`Empting secret JSON config for ${environment}`);
        sh.mkdir(PLAIN_SECRETS_CONFIGS_DIR);
        const envPath = this.#getSecretPath(environment);
        const isLocal = environment === 'local';
        const secretsSchema = this.getSecretsSchema({ allowEmptyValues: !isLocal, environment });
        const mockData = generateMock(secretsSchema);

        sh.exec(`echo '${JSON.stringify(isLocal ? mockData : emptyObjectValues(mockData))}' > ${envPath}`);
        sh.exec(`npx prettier --write ${envPath}`);
    };

    static syncAll = (): void => {
        ENVIRONMENTS_ALL.forEach((environment) => {
            sh.exec(`Syncing Secret JSON config for ${environment}`);
            sh.mkdir(PLAIN_SECRETS_CONFIGS_DIR);

            const envPath = this.#getSecretPath(environment);
            const existingEnvSecret = this.#getSecretJsonObject(environment) ?? {};

            // We want to allow setting default empty values for non local environment
            // to allow users thoroughly check the secrets that have not been properly set.
            // NOTE: We can probably consider just using examples everywhere also just like prod.
            // but will keep an eye on which feels better
            const secretsSchema = this.getSecretsSchema({ allowEmptyValues: true, environment });
            const mockData = generateMock(secretsSchema);

            if (_.isEmpty(existingEnvSecret)) sh.touch(envPath);
            if (environment !== 'local') emptyObjectValues(mockData);

            // Parse the object to filter out stale keys in existing local secret configs
            // This also persists the values of existing secrets
            const mergedObject = secretsSchema.parse(R.mergeDeepLeft(existingEnvSecret, mockData));

            sh.exec(`echo '${JSON.stringify(mergedObject)}' > ${envPath}`);
            sh.exec(`npx prettier --write ${envPath}`);
        });
    };

    static #getSecretJsonObject = (environment: Environment): object | null => {
        const envPath = PlainSecretJsonConfig.#getSecretPath(environment);

        const existingEnvSecret = this.#parseJson<object>(sh.exec(`cat ${envPath}`, { silent: true }).stdout.trim());
        return existingEnvSecret;
    };

    static #getSecretPath = (environment: Environment): string => {
        return path.join(PLAIN_SECRETS_CONFIGS_DIR, `${environment}.json`);
    };

    static #parseJson = <T>(json: string): T | null => {
        try {
            return JSON.parse(json) as T;
        } catch (error) {
            return null;
        }
    };

    static getSecretsSchema = ({
        allowEmptyValues,
        environment,
    }: {
        allowEmptyValues: boolean;
        environment: Environment;
    }) => {
        return getSecretsSample({ allowEmptyValues, environment });
    };
}

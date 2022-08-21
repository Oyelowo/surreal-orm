import { Environment, EnvVarsByCategories } from '../../src/types/ownTypes.js';
import { Simplify } from 'type-fest';
import z from 'zod';
import _ from 'lodash';
import sh from 'shelljs';
import path from 'node:path';
import * as R from 'ramda';
import { getPlainSecretsConfigFilesBaseDir } from '../../src/shared/directoriesManager.js';

type ResourceSecrets<
    TCat extends keyof EnvVarsByCategories,
    TResource extends keyof EnvVarsByCategories[TCat],
    TEnvVars extends keyof EnvVarsByCategories[TCat][TResource]
> = Simplify<Record<TCat, Simplify<Record<TResource, Simplify<Pick<EnvVarsByCategories[TCat][TResource], TEnvVars>>>>>>;
// > = Record<TCat, Record<TResource, Pick<SecretsByCategories[TCat][TResource], TEnvVars>>>;

export type TSecretsKubeManifests = ResourceSecrets<
    'services',
    'graphql-mongo',
    | 'MONGODB_ROOT_PASSWORD'
    | 'MONGODB_PASSWORD'
    | 'MONGODB_ROOT_USERNAME'
    | 'MONGODB_USERNAME'
    | 'REDIS_PASSWORD'
    | 'REDIS_USERNAME'
    | 'OAUTH_GITHUB_CLIENT_ID'
    | 'OAUTH_GITHUB_CLIENT_SECRET'
    | 'OAUTH_GOOGLE_CLIENT_ID'
    | 'OAUTH_GOOGLE_CLIENT_SECRET'
> &
    ResourceSecrets<'services', 'graphql-postgres', 'POSTGRES_PASSWORD' | 'POSTGRES_USERNAME'> &
    ResourceSecrets<
        'services',
        'grpc-mongo',
        'MONGODB_ROOT_PASSWORD' | 'MONGODB_PASSWORD' | 'MONGODB_ROOT_USERNAME' | 'MONGODB_USERNAME'
    > &
    ResourceSecrets<
        'infrastructure',
        'argocd',
        | 'ADMIN_PASSWORD'
        | 'url'
        | 'CONTAINER_REGISTRY_PASSWORD'
        | 'CONTAINER_REGISTRY_USERNAME'
        | 'GITHUB_PASSWORD'
        | 'GITHUB_USERNAME'
    > &
    ResourceSecrets<'infrastructure', 'linkerd-viz', 'PASSWORD'>;

export const getSecretsSample = (): TSecretsKubeManifests => {
    return {
        services: {
            'graphql-mongo': {
                MONGODB_PASSWORD: '',
                MONGODB_ROOT_PASSWORD: '',
                MONGODB_ROOT_USERNAME: '',
                MONGODB_USERNAME: '',
                OAUTH_GITHUB_CLIENT_ID: '',
                OAUTH_GITHUB_CLIENT_SECRET: '',
                OAUTH_GOOGLE_CLIENT_ID: '',
                OAUTH_GOOGLE_CLIENT_SECRET: '',
                REDIS_PASSWORD: '',
                REDIS_USERNAME: '',
            },
            'grpc-mongo': {
                MONGODB_PASSWORD: '',
                MONGODB_ROOT_PASSWORD: '',
                MONGODB_ROOT_USERNAME: '',
                MONGODB_USERNAME: '',
            },
            'graphql-postgres': {
                POSTGRES_USERNAME: '',
                POSTGRES_PASSWORD: '',
            },
        },
        infrastructure: {
            argocd: {
                ADMIN_PASSWORD: '',
                CONTAINER_REGISTRY_PASSWORD: '',
                CONTAINER_REGISTRY_USERNAME: '',
                GITHUB_PASSWORD: '',
                GITHUB_USERNAME: '',
                type: 'git',
                url: 'https://github.com/Oyelowo/modern-distributed-app-template',
            },
            'linkerd-viz': {
                PASSWORD: '',
            },
        },
    };
};

type SchemaOption = {
    requireValues?: boolean;
};
const getSecretsSchema = (option?: SchemaOption) => {
    const { requireValues = true } = option ?? {};
    const string = z.string().min(requireValues ? 1 : 0);

    return z.object(
        _.mapValues(getSecretsSample(), (rCategory) => {
            return z.object(
                _.mapValues(rCategory, (resourceName) => {
                    return z.object(_.mapValues(resourceName, (_secret) => string) as any);
                })
            );
        })
    );
};

function parseSecrets(obj: any, option?: SchemaOption): TSecretsKubeManifests {
    const secretsSchema = getSecretsSchema(option);
    return secretsSchema.strict().parse(obj) as TSecretsKubeManifests;
}

const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();
const ENVIRONMENTS_ALL: Environment[] = ['test', 'local', 'development', 'staging', 'staging'];

export class PlainKubeBuildSecretsManager<
    TCat extends keyof TSecretsKubeManifests,
    TResource extends keyof TSecretsKubeManifests[TCat]
> {
    constructor(private resourceCat: TCat, private resourceName: TResource, private environment: Environment) {}

    getSecrets = (): TSecretsKubeManifests[TCat][TResource] => {
        const allSecretsJson = PlainKubeBuildSecretsManager.#getSecretJsonObject(this.environment);
        return parseSecrets(allSecretsJson, { requireValues: false })[this.resourceCat][this.resourceName];
    };

    static resetValues = (): void => {
        ENVIRONMENTS_ALL.forEach((environment) => {
            sh.echo(`Empting secret JSON config for ${environment}`);
            sh.mkdir('-p', PLAIN_SECRETS_CONFIGS_DIR);
            const envPath = this.#getSecretPath(environment);

            sh.exec(`echo '${JSON.stringify(getSecretsSample())}' > ${envPath}`);
            sh.exec(`npx prettier --write ${envPath}`);
        });
    };

    static syncAll = (): void => {
        ENVIRONMENTS_ALL.forEach((environment) => {
            sh.echo(`Syncing Secret JSON config for ${environment}`);
            sh.mkdir('-p', PLAIN_SECRETS_CONFIGS_DIR);

            const envPath = this.#getSecretPath(environment);
            const existingEnvSecret = this.#getSecretJsonObject(environment) ?? {};

            if (_.isEmpty(existingEnvSecret)) sh.touch(envPath);

            // Allows us to only get valid keys out, so we can parse the merged secrets out.
            // const secretsSchema = getSecretsSchema({ allowEmptyValues: true, environment });
            // Parse the object to filter out stale keys in existing local secret configs
            // This also persists the values of existing secrets
            const mergedObject = parseSecrets(R.mergeDeepLeft(existingEnvSecret, getSecretsSample()), {
                requireValues: false,
            });

            sh.exec(`echo '${JSON.stringify(mergedObject)}' > ${envPath}`);
            sh.exec(`npx prettier --write ${envPath}`);
        });
    };

    static #getSecretJsonObject = (environment: Environment): object | undefined => {
        const envPath = PlainKubeBuildSecretsManager.#getSecretPath(environment);

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
    };
}

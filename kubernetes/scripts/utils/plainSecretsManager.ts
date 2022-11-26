import { ArgoCdEnvVars, Environment, LinkerdVizEnvVars, ResourceName, TResourceCategory } from '../../src/types/ownTypes.js';
import z from 'zod';
import _ from 'lodash';
import sh from 'shelljs';
import path from 'node:path';
import * as R from 'ramda';
import { getPlainSecretsConfigFilesBaseDir } from '../../src/shared/directoriesManager.js';
import { GraphqlSurrealDbEnvVars } from '../../src/services/graphql-surrealdb/settings.js';
import { GrpcSurrealDbEnvVars } from '../../src/services/grpc-surrealdb/settings.js';


export type TResourcesEnvVars = {
    services: {
        'graphql-surrealdb': Partial<GraphqlSurrealDbEnvVars>,
        'grpc-surrealdb': Partial<GrpcSurrealDbEnvVars>,
    };
    infrastructure: {
        'argocd': Partial<ArgoCdEnvVars>,
        'linkerd-viz': Partial<LinkerdVizEnvVars>,
    };
};

export const getSecretsSample = () => {
    return {
        services: {
            'graphql-surrealdb': {
                SURREALDB_ROOT_USERNAME: '',
                SURREALDB_ROOT_PASSWORD: '',
                OAUTH_GITHUB_CLIENT_ID: '',
                OAUTH_GITHUB_CLIENT_SECRET: '',
                OAUTH_GOOGLE_CLIENT_ID: '',
                OAUTH_GOOGLE_CLIENT_SECRET: '',
                REDIS_USERNAME: '',
                REDIS_PASSWORD: '',
                
            }
            ,
            'grpc-surrealdb': {
                SURREALDB_ROOT_USERNAME: '',
                SURREALDB_ROOT_PASSWORD: '',
                OAUTH_GITHUB_CLIENT_ID: '',
                OAUTH_GITHUB_CLIENT_SECRET: '',
                OAUTH_GOOGLE_CLIENT_ID: '',
                OAUTH_GOOGLE_CLIENT_SECRET: '',
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
    } satisfies TResourcesEnvVars satisfies Record<TResourceCategory, Partial<Record<ResourceName, any>>>;
};


export type TSecretsKubeManifests = ReturnType<typeof getSecretsSample>;

type SchemaOption = {
    requireValues?: boolean;
};
const getSecretsSchema = (option: SchemaOption) => {
    const { requireValues = true } = option;
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

function parseSecrets(obj: any, option: SchemaOption): TSecretsKubeManifests {
    const secretsSchema = getSecretsSchema(option);
    return secretsSchema.strict().parse(obj) as TSecretsKubeManifests;
}

const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();
const ENVIRONMENTS_ALL: Environment[] = ['test', 'local', 'development', 'staging', 'staging'];

export class PlainSecretsManager<
    TCat extends keyof TSecretsKubeManifests,
    TResource extends keyof TSecretsKubeManifests[TCat]
> {
    constructor(private resourceCat: TCat, private resourceName: TResource, private environment: Environment) { }

    getSecrets = (): TSecretsKubeManifests[TCat][TResource] => {
        const allSecretsJson = PlainSecretsManager.#getSecretJsonObject(this.environment);
        return parseSecrets(allSecretsJson, { requireValues: false })[this.resourceCat][this.resourceName];
    };

    static resetValues = (): void => {
        ENVIRONMENTS_ALL.forEach((environment) => {
            sh.echo(`Empting secret JSON config for ${environment}`);
            sh.mkdir('-p', PLAIN_SECRETS_CONFIGS_DIR);
            const envPath = this.#getSecretPath(environment);

            sh.exec(`echo '${JSON.stringify(getSecretsSample())}' > ${envPath}`);
            // sh.exec(`npx prettier --write ${envPath}`);
            sh.exec(`make format`);
        });
    };

    static syncAll = (): void => {
        ENVIRONMENTS_ALL.forEach((environment) => {
            sh.echo(`Syncing Secret JSON config for ${environment}`);
            sh.mkdir('-p', PLAIN_SECRETS_CONFIGS_DIR);

            const envPath = this.#getSecretPath(environment);
            const existingEnvSecret = this.#getSecretJsonObject(environment) ?? {};

            if (_.isEmpty(existingEnvSecret)) { sh.touch(envPath); }

            // Allows us to only get valid keys out, so we can parse the merged secrets out.
            // const secretsSchema = getSecretsSchema({ allowEmptyValues: true, environment });
            // Parse the object to filter out stale keys in existing local secret configs
            // This also persists the values of existing secrets
            const mergedObject = parseSecrets(R.mergeDeepLeft(existingEnvSecret, getSecretsSample()), {
                requireValues: false,
            });

            sh.exec(`echo '${JSON.stringify(mergedObject)}' > ${envPath}`);
            // sh.exec(`npx prettier --write ${envPath}`);
            sh.exec(`make format`);
        });
    };

    static #getSecretJsonObject = (environment: Environment): object | undefined => {
        const envPath = PlainSecretsManager.#getSecretPath(environment);

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

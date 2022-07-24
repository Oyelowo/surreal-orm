import { ENVIRONMENTS_ALL } from '../utils/shared';
import path from 'path';
import sh from 'shelljs';
import { getPlainSecretsConfigFilesBaseDir } from '../../resources/shared/manifestsDirectory';
import { Environment, ResourceName } from '../../resources/types/own-types';

export const PLAIN_SECRETS_CONFIGS_DIR = getPlainSecretsConfigFilesBaseDir();
export function getPlainSecretInputFilePath(environment: Environment): string {
    return path.join(PLAIN_SECRETS_CONFIGS_DIR, `${environment}.ts`);
}

import { generateMock } from '@anatine/zod-mock';
import { z } from 'zod';
import R from 'ramda';
import _ from 'lodash';

const secretsSample = z.object({
    'graphql-mongo': z.object({
        MONGODB_USERNAME: z.string().min(1),
        MONGODB_PASSWORD: z.string().min(1),
        MONGODB_ROOT_USERNAME: z.string().min(1),
        MONGODB_ROOT_PASSWORD: z.string().min(1),
        REDIS_USERNAME: z.string().min(1),
        REDIS_PASSWORD: z.string().min(1),
        GITHUB_CLIENT_ID: z.string().min(1),
        GITHUB_CLIENT_SECRET: z.string().min(1),
        GOOGLE_CLIENT_ID: z.string().min(1),
        GOOGLE_CLIENT_SECRET: z.string().min(1),
    }),
    'grpc-mongo': z.object({
        MONGODB_USERNAME: z.string().min(1),
        MONGODB_PASSWORD: z.string().min(1),
        MONGODB_ROOT_USERNAME: z.string().min(1),
        MONGODB_ROOT_PASSWORD: z.string().min(1),
    }),
    'graphql-postgres': z.object({
        POSTGRES_USERNAME: z.string().min(1),
        POSTGRES_PASSWORD: z.string().min(1),
    }),
    'react-web': z.object({}),
    argocd: z.object({
        ADMIN_PASSWORD: z.string().min(1),
        type: z
            .string()
            .min(1)
            .transform(() => 'git'),
        url: z
            .string()
            .min(1)
            .transform((_item) => 'https://github.com/Oyelowo/modern-distributed-app-template'),
        username: z
            .string()
            .min(1)
            .transform(() => 'Oyelowo'),
        password: z.string().min(1),
    }),
    'argocd-applications-children-infrastructure': z.object({}),
    'argocd-applications-children-services': z.object({}),
    'argocd-applications-parents': z.object({}),
    'cert-manager': z.object({}),
    linkerd: z.object({}),
    'linkerd-viz': z.object({
        PASSWORD: z.string().min(1),
    }),
    namespaces: z.object({}),
    'nginx-ingress': z.object({}),
    'sealed-secrets': z.object({}),
});

type TSecretJson = z.infer<typeof secretsSample>;

const mockData = generateMock(secretsSample);

function empty(object: any) {
    Object.keys(object).forEach((k) => {
        if (object[k] && typeof object[k] === 'object') {
            return empty(object[k]);
        }
        object[k] = '';
    });
}

class PlainSecretJsonConfig {
    constructor(private resourceName: ResourceName, private environment: Environment) { }

    getSecretsForResource<App extends ResourceName>(): TSecretJson[App] {
        PlainSecretJsonConfig.sync();

        return secretsSample.parse(PlainSecretJsonConfig.#getSecretJsonObject(this.environment))[
            this.resourceName
        ];
    }

    static sync = () => {
        ENVIRONMENTS_ALL.forEach((env) => {
            sh.exec(`Syncing Secret JSON config for ${env}`)
            sh.mkdir(PLAIN_SECRETS_CONFIGS_DIR);

            const envPath = this.#getSecretPath(env);
            const existingEnvSecret = this.#getSecretJsonObject(env)

            if (_.isEmpty(existingEnvSecret)) sh.touch(envPath);
            // if (env !== 'local') empty(mockData);

            // Parse the object to filter out stale keys in existing local secret configs
            // This also persists the values of existing secrets
            const mergedObject = secretsSample.parse(R.mergeDeepLeft(existingEnvSecret, mockData));

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
}

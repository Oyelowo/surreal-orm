import {
    appEnvironmentsSchema,
    Environment,
    EnvVarsCommon,
    InfrastructureName,
    ResourceCategory,
    ServiceName,
} from './ownTypes.js';
import { Simplify, SnakeCase } from 'type-fest';
import z from 'zod';
import _ from 'lodash';

type GraphqlMongo = EnvVarsCommon<'graphql-mongo', 'applications', 'app' | 'mongodb' | 'oauth' | 'redis'>;

type GraphqlPostgres = EnvVarsCommon<'graphql-postgres', 'applications', 'app' | 'postgresdb'>;

type GrpcMongo = EnvVarsCommon<'grpc-mongo', 'applications', 'app' | 'mongodb'>;

type RecordServiceEnvVars<N extends ServiceName, EnvVars> = Record<N, EnvVars>;
type RecordInfraEnvVars<N extends InfrastructureName, EnvVars> = Record<N, EnvVars>;

type ArgoCd = {
    ADMIN_PASSWORD: string;
    type: 'git';
    url: 'https://github.com/Oyelowo/modern-distributed-app-template';
    username: string;
    password: string;
};

type LinkerdViz = {
    PASSWORD: string;
};

// Creates Record<ResourceName, Record<EnvVarName, string>>
type ServicesEnvVars = Simplify<
    RecordServiceEnvVars<'graphql-mongo', GraphqlMongo> &
        RecordServiceEnvVars<'graphql-postgres', GraphqlPostgres> &
        RecordServiceEnvVars<'grpc-mongo', GrpcMongo> &
        RecordServiceEnvVars<'react-web', null>
>;
type InfrastructureEnvVars = Simplify<
    RecordInfraEnvVars<'argocd', ArgoCd> & RecordInfraEnvVars<'linkerd-viz', LinkerdViz>
>;

type EnvVarsByCategory<RCat extends ResourceCategory, V> = Record<RCat, V>;

export type EnvVarsByResourceCategory = EnvVarsByCategory<'infrastructure', InfrastructureEnvVars> &
    EnvVarsByCategory<'services', ServicesEnvVars>;

type Stringified<T> = Extract<T, string>;
/** Generates the format in all caps:  \<ResourceCategory>\__\<ResourceName>\__\<EnvironmentVariableNaame>
 * @example SERVICES__GRAPHQL_MONGO__REDIS_PASSWORD */
type CreateEnvCarsCreator<
    RCat extends ResourceCategory,
    ResourceEnvVar,
    RName extends keyof ResourceEnvVar,
    EnvVarNames extends keyof ResourceEnvVar[RName]
> = Record<
    Uppercase<`${RCat}__${SnakeCase<Stringified<RName>>}__${keyof Pick<
        ResourceEnvVar[RName],
        Stringified<EnvVarNames>
    >}`>,
    string
>;

type SelectSecretsFromServicesEnvVars<
    RName extends keyof ServicesEnvVars,
    EnvVarNames extends keyof ServicesEnvVars[RName]
> = CreateEnvCarsCreator<'services', ServicesEnvVars, RName, EnvVarNames>;

type SelectSecretsFromInfraEnvVars<
    RName extends keyof InfrastructureEnvVars,
    EnvVarNames extends keyof InfrastructureEnvVars[RName]
> = CreateEnvCarsCreator<'infrastructure', InfrastructureEnvVars, RName, EnvVarNames>;

// The service, Selected Environment variables that would be passed when generating kubernetes secrets manifests
type SelectedSecretsEnvVars = Simplify<
    {
        ENVIRONMENT: Environment;
        // This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
        IMAGE_TAG_REACT_WEB: string;
        IMAGE_TAG_GRAPHQL_MONGO: string;
        IMAGE_TAG_GRPC_MONGO: string;
        IMAGE_TAG_GRAPHQL_POSTGRES: string;
    } & SelectSecretsFromServicesEnvVars<
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
        SelectSecretsFromServicesEnvVars<'graphql-postgres', 'POSTGRES_PASSWORD' | 'POSTGRES_USERNAME'> &
        SelectSecretsFromServicesEnvVars<
            'grpc-mongo',
            'MONGODB_ROOT_PASSWORD' | 'MONGODB_PASSWORD' | 'MONGODB_ROOT_USERNAME' | 'MONGODB_USERNAME'
        > &
        SelectSecretsFromInfraEnvVars<'argocd', 'ADMIN_PASSWORD' | 'password' | 'type' | 'url' | 'username'> &
        SelectSecretsFromInfraEnvVars<'linkerd-viz', 'PASSWORD'>
>;

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

    const secretsSample1: Record<keyof SelectedSecretsEnvVars, z.ZodType<any, any>> = {
        ENVIRONMENT: appEnvironmentsSchema,
        // This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
        IMAGE_TAG_REACT_WEB: string,
        IMAGE_TAG_GRAPHQL_MONGO: string,
        IMAGE_TAG_GRPC_MONGO: string,
        IMAGE_TAG_GRAPHQL_POSTGRES: string,
        SERVICES__GRAPHQL_MONGO__MONGODB_USERNAME: string,
        SERVICES__GRAPHQL_MONGO__MONGODB_PASSWORD: string,
        SERVICES__GRAPHQL_MONGO__MONGODB_ROOT_USERNAME: string,
        SERVICES__GRAPHQL_MONGO__MONGODB_ROOT_PASSWORD: string,
        SERVICES__GRAPHQL_MONGO__REDIS_USERNAME: string,
        SERVICES__GRAPHQL_MONGO__REDIS_PASSWORD: string,
        SERVICES__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_ID: string,
        SERVICES__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_SECRET: string,
        SERVICES__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_ID: string,
        SERVICES__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_SECRET: string,

        SERVICES__GRPC_MONGO__MONGODB_USERNAME: string,
        SERVICES__GRPC_MONGO__MONGODB_PASSWORD: string,
        SERVICES__GRPC_MONGO__MONGODB_ROOT_USERNAME: string,
        SERVICES__GRPC_MONGO__MONGODB_ROOT_PASSWORD: string,

        SERVICES__GRAPHQL_POSTGRES__POSTGRES_USERNAME: string,
        SERVICES__GRAPHQL_POSTGRES__POSTGRES_PASSWORD: string,

        INFRASTRUCTURE__ARGOCD__ADMIN_PASSWORD: string,
        INFRASTRUCTURE__ARGOCD__TYPE: stringNoDefault.transform(getDefault('git')),
        INFRASTRUCTURE__ARGOCD__URL: stringNoDefault.transform(
            getDefault('https://github.com/Oyelowo/modern-distributed-app-template')
        ),
        INFRASTRUCTURE__ARGOCD__USERNAME: stringNoDefault.transform(getDefault('Oyelowo')),
        INFRASTRUCTURE__ARGOCD__PASSWORD: string,

        INFRASTRUCTURE__LINKERD_VIZ__PASSWORD: string,
    };

    return z.object(secretsSample1);
};

// type Momo = Record<Uppercase<`${TServices}__${SnakeCase<N>}__`>, string>>
const envv: SelectedSecretsEnvVars = {
    ENVIRONMENT: 'local',
    // This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
    IMAGE_TAG_REACT_WEB: '',
    IMAGE_TAG_GRAPHQL_MONGO: '',
    IMAGE_TAG_GRPC_MONGO: '',
    IMAGE_TAG_GRAPHQL_POSTGRES: '',

    SERVICES__GRAPHQL_MONGO__MONGODB_PASSWORD: '',
    SERVICES__GRAPHQL_MONGO__MONGODB_ROOT_PASSWORD: '',
    SERVICES__GRAPHQL_MONGO__MONGODB_ROOT_USERNAME: '',
    SERVICES__GRAPHQL_MONGO__MONGODB_USERNAME: '',
    SERVICES__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_ID: '',
    SERVICES__GRAPHQL_MONGO__OAUTH_GITHUB_CLIENT_SECRET: '',
    SERVICES__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_ID: '',
    SERVICES__GRAPHQL_MONGO__OAUTH_GOOGLE_CLIENT_SECRET: '',
    SERVICES__GRAPHQL_MONGO__REDIS_PASSWORD: '',
    SERVICES__GRAPHQL_MONGO__REDIS_USERNAME: '',

    SERVICES__GRPC_MONGO__MONGODB_PASSWORD: '',
    SERVICES__GRPC_MONGO__MONGODB_ROOT_PASSWORD: '',
    SERVICES__GRPC_MONGO__MONGODB_ROOT_USERNAME: '',
    SERVICES__GRPC_MONGO__MONGODB_USERNAME: '',

    SERVICES__GRAPHQL_POSTGRES__POSTGRES_PASSWORD: '',
    SERVICES__GRAPHQL_POSTGRES__POSTGRES_USERNAME: '',

    INFRASTRUCTURE__ARGOCD__ADMIN_PASSWORD: '',
    INFRASTRUCTURE__ARGOCD__PASSWORD: '',
    INFRASTRUCTURE__ARGOCD__TYPE: '',
    INFRASTRUCTURE__ARGOCD__URL: '',
    INFRASTRUCTURE__ARGOCD__USERNAME: '',

    INFRASTRUCTURE__LINKERD_VIZ__PASSWORD: '',
};

// export const getEnvVarsForKubeManifestGenerator = (): SelectedSecretsEnvVars => {
export const getEnvVarsForKubeManifestGenerator = () => {
    const environment = appEnvironmentsSchema.parse(process.env);
    const schema = getSecretsSchema({ allowEmptyValues: true, environment });
    return schema.parse(process.env);
};

import {
    EnvVarsCommon,
    InfrastructureName,
    ResourceCategory,
    ServiceName,
    TInfrastructure,
    TServices,
} from './ownTypes.js';
import { Simplify, SnakeCase } from 'type-fest';

type GraphqlMongo = EnvVarsCommon<'graphql-mongo', 'applications', 'app' | 'mongodb' | 'oauth' | 'redis'>;

type GraphqlPostgres = EnvVarsCommon<'graphql-postgres', 'applications', 'app' | 'postgresdb'>;

type GrpcMongo = EnvVarsCommon<'grpc-mongo', 'applications', 'app' | 'mongodb'>;

type RecordServiceEnvVars<N extends ServiceName, EnvVars> = Record<N, EnvVars>;
type RecordInfraEnvVars<N extends InfrastructureName, EnvVars> = Record<N, EnvVars>;

// type CatWithEnv<RCat extends ResourceCategory, V> = Record<RCat, V>;
// type L = CatWithEnv<"services", Enva>

type ArgoCd = {
    ADMIN_PASSWORD: string;
    type: 'git';
    url: 'https://github.com/Oyelowo/modern-distributed-app-template';
    username: string;
    password: string;
};

type ServiceEnvVars = Simplify<
    RecordServiceEnvVars<'graphql-mongo', GraphqlMongo> &
    RecordServiceEnvVars<'graphql-postgres', GraphqlPostgres> &
    RecordServiceEnvVars<'grpc-mongo', GrpcMongo>
>;
type InfrastructureEnvVars = Simplify<RecordInfraEnvVars<'argocd', ArgoCd>>;

type Secc = {
    infrastructure: InfrastructureEnvVars;
    services: ServiceEnvVars;
};

// type EnvVarsSecrets<T, K extends keyof T[keyof T], E extends keyof T[keyof T][K]> = Record<Uppercase<`${TServices}__${SnakeCase<Extract<K, string>>}__${keyof Pick<T[keyof T][K], Extract<E, string>>}`>, string>;
type Stringified<T> = Extract<T, string>;
/** Generates the format in all caps:  \<ResourceCategory>\__\<ResourceName>\__\<EnvironmentVariableNaame>
 * @example SERVICES__GRAPHQL_MONGO__REDIS_PASSWORD */
type CreateEnvCarsCreator<
    RCat extends ResourceCategory,
    ResourceEnvVar,
    RName extends keyof ResourceEnvVar,
    EnvVarNames extends keyof ResourceEnvVar[RName],
    > = Record<
        Uppercase<`${RCat}__${SnakeCase<Stringified<RName>>}__${keyof Pick<ResourceEnvVar[RName], Stringified<EnvVarNames>>}`>,
        string
    >;

type SelectSecretsFromServicesEnvVars<RName extends keyof ServiceEnvVars, EnvVarNames extends keyof ServiceEnvVars[RName]> = CreateEnvCarsCreator<
    'services',
    ServiceEnvVars,
    RName,
    EnvVarNames
>;
type SelectSecretsFromInfraEnvVars<RName extends keyof InfrastructureEnvVars, EnvVarNames extends keyof InfrastructureEnvVars[RName]> = CreateEnvCarsCreator<
    'infrastructure',
    InfrastructureEnvVars,
    RName,
    EnvVarNames
>;


// The service, Selected Environment variables that would be passed when generating kubernetes secrets manifests
type SelectedSecretsEnvVars = SelectSecretsFromServicesEnvVars<
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
    SelectSecretsFromInfraEnvVars<'argocd', 'ADMIN_PASSWORD' | 'password' | 'type' | 'url' | 'username'>;

// type Momo = Record<Uppercase<`${TServices}__${SnakeCase<N>}__`>, string>>
const envv: SelectedSecretsEnvVars = {
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

    // SERVICES__ARGOCD__ADMIN_PASSWORD
};

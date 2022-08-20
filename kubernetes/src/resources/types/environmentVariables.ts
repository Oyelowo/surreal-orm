import {
    Environment,
    EnvVarsCommon,
    InfrastructureName,
    ResourceCategory,
    ServiceName,
    TServices,
} from './ownTypes.js';
import { Simplify, SnakeCase } from 'type-fest';
import z from 'zod';
import _ from 'lodash';

// This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
//     IMAGE_TAG_REACT_WEB: string;
// IMAGE_TAG_GRAPHQL_MONGO: string;
// IMAGE_TAG_GRPC_MONGO: string;
// IMAGE_TAG_GRAPHQL_POSTGRES: string;
type GraphqlMongo = EnvVarsCommon<'graphql-mongo', 'applications', 'app' | 'mongodb' | 'oauth' | 'redis'>;

type GraphqlPostgres = EnvVarsCommon<'graphql-postgres', 'applications', 'app' | 'postgresdb'>;

type GrpcMongo = EnvVarsCommon<'grpc-mongo', 'applications', 'app' | 'mongodb'>;

type ReactWeb = EnvVarsCommon<'grpc-mongo', 'applications', 'app'>;

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
    RecordServiceEnvVars<'react-web', ReactWeb>
>;
type InfrastructureEnvVars = Simplify<
    RecordInfraEnvVars<'argocd', ArgoCd> & RecordInfraEnvVars<'linkerd-viz', LinkerdViz>
>;

type EnvVarsByCategory<RCat extends ResourceCategory, V> = Record<RCat, V>;

export type EnvVarsByResourceCategory = EnvVarsByCategory<'infrastructure', InfrastructureEnvVars> &
    EnvVarsByCategory<'services', ServicesEnvVars>;

type Stringified<T> = Extract<T, string>;
type PrefixEnvVar<RCat extends ResourceCategory, RName, EnvVarName extends string> = Uppercase<`${RCat}__${SnakeCase<
    Stringified<RName>
>}__${EnvVarName}`>;

/** Generates the format in all caps:  \<ResourceCategory>\__\<ResourceName>\__\<EnvironmentVariableNaame>
 * @example SERVICES__GRAPHQL_MONGO__REDIS_PASSWORD */
type CreateEnvCarsCreator<
    RCat extends ResourceCategory,
    ResourceCategoryEnvVar,
    RName extends keyof ResourceCategoryEnvVar,
    EnvVarNames extends keyof ResourceCategoryEnvVar[RName]
    > = Record<PrefixEnvVar<RCat, RName, keyof Pick<ResourceCategoryEnvVar[RName], Stringified<EnvVarNames>>>, string>;

type SelectSecretsFromServicesEnvVars<
    RName extends keyof ServicesEnvVars,
    EnvVarNames extends keyof ServicesEnvVars[RName]
    > = CreateEnvCarsCreator<'services', ServicesEnvVars, RName, EnvVarNames>;

type SelectSecretsFromInfraEnvVars<
    RName extends keyof InfrastructureEnvVars,
    EnvVarNames extends keyof InfrastructureEnvVars[RName]
    > = CreateEnvCarsCreator<'infrastructure', InfrastructureEnvVars, RName, EnvVarNames>;


const imageTagsSchema: Record<PrefixEnvVar<TServices, ServiceName, "IMAGE_TAG">, z.ZodString> = {
    // This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
    SERVICES__GRAPHQL_MONGO__IMAGE_TAG: z.string().min(1),
    SERVICES__GRAPHQL_POSTGRES__IMAGE_TAG: z.string().min(1),
    SERVICES__GRPC_MONGO__IMAGE_TAG: z.string().min(1),
    SERVICES__REACT_WEB__IMAGE_TAG: z.string().min(1),
}

// CONSIDER: Move this into helpers
export const imageTagsObjectValidator = z.object(imageTagsSchema);
export type ImageTags = z.infer<typeof imageTagsObjectValidator>;

// The service, Selected Environment variables that would be passed when generating kubernetes secrets manifests
type SelectedSecretsEnvVars = Simplify<
    {
        ENVIRONMENT: Environment;
    } & ImageTags &
    SelectSecretsFromServicesEnvVars<
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

// type Momo = Record<Uppercase<`${TServices}__${SnakeCase<N>}__`>, string>>
export const kubeBuildEnvVarsSample: SelectedSecretsEnvVars = {
    ENVIRONMENT: 'local',
    // This is provided fro, within the CI pipeline where the manifests are generated and pushed to the repo
    SERVICES__GRAPHQL_MONGO__IMAGE_TAG: "",
    SERVICES__GRAPHQL_POSTGRES__IMAGE_TAG: "",
    SERVICES__GRPC_MONGO__IMAGE_TAG: "",
    SERVICES__REACT_WEB__IMAGE_TAG: "",

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

export const getSecretsSchema = ({ allowEmptyValues }: { allowEmptyValues: boolean }) => {
    const stringNoDefault = z.string().min(allowEmptyValues ? 0 : 1);

    const secretsSample1: Record<keyof SelectedSecretsEnvVars, z.ZodString> = _.mapValues(
        kubeBuildEnvVarsSample,
        (_) => stringNoDefault
    );

    return z.object(secretsSample1);
};

// export const getEnvVarsForKubeManifestGenerator = (): SelectedSecretsEnvVars => {
export const getEnvVarsForKubeManifestGenerator = (): SelectedSecretsEnvVars => {
    // console.log("XXX", process.)
    // const environment = appEnvironmentsSchema.parse(process.env.ENVIRONMENT);
    const schema = getSecretsSchema({ allowEmptyValues: true });
    return schema.parse(process.env) as SelectedSecretsEnvVars;
};

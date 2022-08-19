import type { ValueOf, Simplify, UnionToIntersection } from 'type-fest';

import * as z from 'zod';
import { Namespace } from '../infrastructure/namespaces/util.js';
import { EnvVarsByResourceCategory } from './environmentVariables.js';
export const appEnvironmentsSchema = z.union([
    z.literal('test'),
    z.literal('local'),
    z.literal('development'),
    z.literal('staging'),
    z.literal('production'),
]);

export type Environment = z.infer<typeof appEnvironmentsSchema>;
// This might change but make it the environment for now.
export type NamespaceOfApps = Namespace;

export type Memory = `${number}${'E' | 'P' | 'T' | 'G' | 'M' | 'k' | 'm' | 'Ei' | 'Pi' | 'Ti' | 'Gi' | 'Mi' | 'Ki'}`;

export type CPU = `${number}${'m'}`;

export const ServiceNamesSchema = z.union([
    z.literal('graphql-mongo'),
    z.literal('graphql-postgres'),
    z.literal('grpc-mongo'),
    z.literal('react-web'),
]);
const ServiceNames = ['graphql-mongo', 'graphql-postgres', 'grpc-mongo', 'react-web'] as const;
export type ServiceName = typeof ServiceNames[number];

const infrastructure = 'infrastructure';
const services = 'services';
export type TInfrastructure = typeof infrastructure;
export type TServices = typeof services;
export type ResourceCategory = TInfrastructure | TServices;

export type ResourcePaths = `${TInfrastructure}/${InfrastructureName}` | `${TServices}/${ServiceName}`;

export const ArgocdAppResourceNameSchema = z.union([
    z.literal(`argocd-applications-children-${infrastructure}`),
    z.literal(`argocd-applications-children-${services}`),
    z.literal('argocd-applications-parents'),
]);

export type ArgocdAppResourceName = z.infer<typeof ArgocdAppResourceNameSchema>;

export const InfrastructureNamesSchema = z.union([
    z.literal('namespaces'),
    z.literal('sealed-secrets'),
    z.literal('cert-manager'),
    z.literal('nginx-ingress'),
    z.literal('linkerd'),
    z.literal('linkerd-viz'),
    z.literal('argocd'),
]);

export type InfrastructureName = z.infer<typeof InfrastructureNamesSchema> | ArgocdAppResourceName;

export const ResourceNameSchema = z.union([ServiceNamesSchema, InfrastructureNamesSchema, ArgocdAppResourceNameSchema]);

// A resource can have multiple kubernetes objects/resources e.g linkerd
// e.g linkerd can have different
export type ResourceName = z.infer<typeof ResourceNameSchema>;
export type NoUnion<T, U = T> = T extends U ? ([U] extends [T] ? T : never) : never;

export interface Settings<N extends ServiceName> {
    requestMemory: Memory;
    requestCpu: CPU;
    limitMemory: Memory;
    limitCpu: CPU;
    replicaCount: number;
    host: string;
    image: `ghcr.io/oyelowo/${N}:${string}`;
    readinessProbePort?: number;
}

// make all properties optional recursively including nested objects.
// keep in mind that this should be used on json / plain objects only.
// otherwise, it will make class methods optional as well.
export type DeepPartial<T> = T extends object ? { [K in keyof T]?: DeepPartial<T[K]> } : T;

export const STORAGE_CLASS = 'linode-block-storage-retain';
export type MongoDbEnvVars<DBN extends `${ServiceName}-database`, NS extends NamespaceOfApps> = {
    MONGODB_NAME: DBN;
    MONGODB_USERNAME: string;
    MONGODB_PASSWORD: string;
    MONGODB_HOST: `${DBN}.${NS}`;
    MONGODB_PORT: '27017';
    MONGODB_SERVICE_NAME: DBN;
    MONGODB_STORAGE_CLASS: typeof STORAGE_CLASS;
    MONGODB_ROOT_USERNAME: string;
    MONGODB_ROOT_PASSWORD: string;
};

export type PostgresDbEnvVars<DBN extends `${ServiceName}-database`, NS extends NamespaceOfApps> = {
    POSTGRES_NAME: DBN;
    POSTGRES_DATABASE_NAME: DBN;
    POSTGRES_USERNAME: string;
    POSTGRES_PASSWORD: string;
    POSTGRES_HOST: `${DBN}.${NS}`;
    POSTGRES_PORT: '5432';
    POSTGRES_SERVICE_NAME: DBN;
    POSTGRES_STORAGE_CLASS: string;
};

type RedisServiceNameMaster<N extends `${ServiceName}-redis`> = `${N}-master`;

export type RedisDbEnvVars<N extends `${ServiceName}-redis`, NS extends NamespaceOfApps> = {
    REDIS_USERNAME: string;
    REDIS_PASSWORD: string;
    REDIS_HOST: `${RedisServiceNameMaster<N>}.${NS}`; // The application will also need this
    REDIS_SERVICE_NAME: N; // THIS is used in redis helm chart config itself which adds a suffix(e.g master)
    REDIS_SERVICE_NAME_MASTER: RedisServiceNameMaster<N>;
    REDIS_PORT: '6379';
};

export type AppEnvVars = {
    APP_ENVIRONMENT: Environment;
    APP_HOST: '0.0.0.0';
    APP_PORT: '8000' | '50051' | '3000';
    // the url of the ingress e.g oyelowo.com // localhost:8080 (for local dev)
    APP_EXTERNAL_BASE_URL: string;
};

export type OauthEnvVars = {
    OAUTH_GITHUB_CLIENT_ID: string;
    OAUTH_GITHUB_CLIENT_SECRET: string;
    OAUTH_GOOGLE_CLIENT_ID: string;
    OAUTH_GOOGLE_CLIENT_SECRET: string;
};

type EnvVariables<S extends ServiceName, NS extends NamespaceOfApps> = {
    mongodb: MongoDbEnvVars<`${S}-database`, NS>;
    postgresdb: PostgresDbEnvVars<`${S}-database`, NS>;
    redis: RedisDbEnvVars<`${S}-redis`, NS>;
    app: AppEnvVars;
    oauth: OauthEnvVars;
    // others: T;
};

/**
 *  @example
 * @argument type Kamo = EnvironmentVariablesCommon<ServiceName, Namespace, [ListOfSelectedEnvVars]>
 * type Kamo = EnvironmentVariablesCommon<"graphql-mongo", "applications", ["app", "mongodb"]>
 */
export type EnvVarsCommon<
    N extends ServiceName,
    NS extends NamespaceOfApps,
    SelectedEnvKey extends keyof EnvVariables<N, NS>
> = Simplify<UnionToIntersection<ValueOf<Pick<EnvVariables<N, NS>, SelectedEnvKey>>>>;

export type AppConfigs<N extends ServiceName, NS extends NamespaceOfApps> = {
    kubeConfig: Settings<N>;
    envVars: EnvVarsByResourceCategory['services'][N];
    metadata: {
        name: N;
        namespace: NS;
    };
};

import * as z from 'zod';
import { Namespace } from '../infrastructure/namespaces/util.js';
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
    z.literal('surrealdb'), // This is a database layer relies on other persistent layers: SurrealDB is deployed standalone as a logic layer over TiKV. It can also use in-memory DB
    z.literal('graphql-surrealdb'), // This is an application/server layer
    z.literal('grpc-surrealdb'),
    z.literal('react-web'),
]);

export type ServiceName = z.infer<typeof ServiceNamesSchema>;

const infrastructure = 'infrastructure';
const services = 'services';
export type TInfrastructure = typeof infrastructure;
export type TServices = typeof services;
export type TResourceCategory = TInfrastructure | TServices;

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
    z.literal('tikv-operator'),
    z.literal('seaweedfs'),
    z.literal('tidis'),
    z.literal('rook-ceph'),
    z.literal('metalb'),
    z.literal('fluvio-sys'),
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
    image: `ghcr.io/oyelowo/${N}:${string}` | 'surrealdb/surrealdb:1.0.0-beta.8';
    readinessProbePort?: number;
    command?: string[];
    commandArgs?: string[];
}

// make all properties optional recursively including nested objects.
// keep in mind that this should be used on json / plain objects only.
// otherwise, it will make class methods optional as well.
export type DeepPartial<T> = T extends object ? { [K in keyof T]?: DeepPartial<T[K]> } : T;

export const STORAGE_CLASS = 'linode-block-storage-retain';
type SurrealDbName = 'surrealdb';
export type SurrealDbEnvVars<NS extends NamespaceOfApps> = {
    SURREALDB_NAME: SurrealDbName;
    // SURREALDB_USERNAME: string;
    // SURREALDB_PASSWORD: string;
    SURREALDB_HOST: `${SurrealDbName}.${NS}`;
    SURREALDB_PORT: '8000';
    SURREALDB_SERVICE_NAME: SurrealDbName;
    // SURREALDB_STORAGE_CLASS: typeof STORAGE_CLASS;
    SURREALDB_ROOT_USERNAME: string;
    SURREALDB_ROOT_PASSWORD: string;
};

type TikvDbName = 'tikv';
export type TikVDbEnvVars<NS extends NamespaceOfApps> = {
    TIKV_NAME: TikvDbName;
    // TIKV_USERNAME: string;
    // TIKV_PASSWORD: string;
    TIKV_HOST: `${TikvDbName}-pd.${NS}`;
    TIKV_PORT: '2379';
    TIKV_SERVICE_NAME: TikvDbName;
    TIKV_STORAGE_CLASS: typeof STORAGE_CLASS;
};

type RedisDbName = `redis`;
type RedisServiceNameMaster = `${RedisDbName}-master`;
export type RedisDbEnvVars<NS extends NamespaceOfApps> = {
    REDIS_USERNAME: string;
    REDIS_PASSWORD: string;
    REDIS_HOST: `${RedisServiceNameMaster}.${NS}`; // The application will also need this
    REDIS_SERVICE_NAME: RedisDbName; // THIS is used in redis helm chart config itself which adds a suffix(e.g master)
    REDIS_SERVICE_NAME_MASTER: RedisServiceNameMaster;
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

export type ArgoCdEnvVars = {
    ADMIN_PASSWORD: string;
    type: 'git';
    url: 'https://github.com/Oyelowo/modern-distributed-app-template';
    // ARGO_CD_PASSWORD: string;
    /* Github credentials */
    GITHUB_USERNAME: string;
    GITHUB_PASSWORD: string;

    /* Using github registry for now */
    CONTAINER_REGISTRY_USERNAME: string;
    CONTAINER_REGISTRY_PASSWORD: string;
};

export type LinkerdVizEnvVars = {
    PASSWORD: string;
};

//  Application configurations/Settings which is passed to deployment
export type AppConfigs<N extends ServiceName, NS extends NamespaceOfApps, EnvVars extends Record<string, string>> = {
    kubeConfig: Settings<N>;
    envVars: EnvVars;
    metadata: {
        name: N;
        namespace: NS;
    };
};

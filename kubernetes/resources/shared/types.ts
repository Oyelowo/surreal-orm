export type Environemt = "local" | "development" | "staging" | "production";
// This might change but make it the environment for now.
export type Namespace = Environemt;

export type Memory = `${number}${
  | "E"
  | "P"
  | "T"
  | "G"
  | "M"
  | "k"
  | "m"
  | "Ei"
  | "Pi"
  | "Ti"
  | "Gi"
  | "Mi"
  | "Ki"}`;

export type CPU = `${number}${"m"}`;

export type AppName =
  | "graphql-mongo"
  | "graphql-postgres"
  | "grpc-mongo"
  | "react-web";

export interface Settings<TAppName extends AppName> {
  resourceName: TAppName;
  requestMemory: Memory;
  requestCpu: CPU;
  limitMemory: Memory;
  limitCpu: CPU;
  host: string;
  image: `oyelowo/${TAppName}`;
}

export type RecursivePartial<T> = {
  [P in keyof T]?: T[P] extends (infer U)[]
    ? RecursivePartial<U>[]
    : T[P] extends object | undefined
    ? RecursivePartial<T[P]>
    : T[P];
};

// make all properties optional recursively including nested objects.
// keep in mind that this should be used on json / plain objects only.
// otherwise, it will make class methods optional as well.
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends Array<infer I>
    ? Array<DeepPartial<I>>
    : DeepPartial<T[P]>;
};

type MongoDb = "mongodb";
type PostgresDb = "postgresdb";

type DBType = MongoDb | PostgresDb;

export type MongoDbEnvVars<
  DbName extends `${AppName}-database`,
  TNamespace extends Namespace
> = {
  dbType: MongoDb;
  MONGODB_NAME: DbName;
  MONGODB_USERNAME: string;
  MONGODB_PASSWORD: string;
  MONGODB_HOST: `${DbName}.${TNamespace}`;
  MONGODB_PORT: "27017";
  MONGODB_SERVICE_NAME: DbName;
};

export type PostgresDbEnvVars<
  DbName extends `${AppName}-database`,
  TNamespace extends Namespace
> = {
  dbType: PostgresDb;
  POSTGRES_NAME: DbName;
  POSTGRES_DATABASE_NAME: DbName;
  POSTGRES_USERNAME: "postgres";
  POSTGRES_PASSWORD: string;
  POSTGRES_HOST: `${DbName}.${TNamespace}`;
  POSTGRES_PORT: "5432";
  POSTGRES_SERVICE_NAME: DbName;
};

type DatabaseEnvVars<
  DbName extends `${AppName}-database`,
  TNamespace extends Namespace
> = MongoDbEnvVars<DbName, TNamespace> | PostgresDbEnvVars<DbName, TNamespace>;

export type AppEnvVars<
  TAppName extends AppName,
  TNamespace extends Namespace
> = {
  APP_ENVIRONMENT: Environemt;
  APP_HOST: "0.0.0.0";
  APP_PORT: "8000" | `50051`;
} & DatabaseEnvVars<`${TAppName}-database`, TNamespace>;

type EnvironmentVariables<
  TAppName extends AppName,
  TNamespace extends Namespace,
  TDBType extends DBType
> = Extract<AppEnvVars<TAppName, TNamespace>, { dbType: TDBType }>;

export type AppConfigs<
  TAppName extends AppName,
  TDBType extends DBType,
  TNamespace extends Namespace
> = {
  kubeConfig: Settings<TAppName>;
  envVars: Omit<EnvironmentVariables<TAppName, TNamespace, TDBType>, "dbType">;
};

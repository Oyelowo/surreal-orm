export type Environemt = "local" | "development" | "staging" | "production";
// This might change but make it the environment for now.
export type NamespaceOfApps = Environemt;

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
type DoesNotHaveDb = "doesNotHaveDb";

export type DBType = MongoDb | PostgresDb | DoesNotHaveDb;

export type MongoDbEnvVars<
  DBN extends `${AppName}-database`,
  NS extends NamespaceOfApps
> = {
  dbType: MongoDb;
  MONGODB_NAME: DBN;
  MONGODB_USERNAME: string;
  MONGODB_PASSWORD: string;
  MONGODB_HOST: `${DBN}.${NS}`;
  MONGODB_PORT: "27017";
  MONGODB_SERVICE_NAME: DBN;
};

export type PostgresDbEnvVars<
  DBN extends `${AppName}-database`,
  NS extends NamespaceOfApps
> = {
  dbType: PostgresDb;
  POSTGRES_NAME: DBN;
  POSTGRES_DATABASE_NAME: DBN;
  POSTGRES_USERNAME: "postgres";
  POSTGRES_PASSWORD: string;
  POSTGRES_HOST: `${DBN}.${NS}`;
  POSTGRES_PORT: "5432";
  POSTGRES_SERVICE_NAME: DBN;
};

type DatabaseEnvVars<
  DBN extends `${AppName}-database`,
  NS extends NamespaceOfApps
> =
  | MongoDbEnvVars<DBN, NS>
  | PostgresDbEnvVars<DBN, NS>
  | { dbType: DoesNotHaveDb };

export type AppEnvVars<AN extends AppName, NS extends NamespaceOfApps> = {
  APP_ENVIRONMENT: Environemt;
  APP_HOST: "0.0.0.0";
  APP_PORT: "8000" | "50051" | "3000";
} & DatabaseEnvVars<`${AN}-database`, NS>;

type EnvironmentVariables<
  AN extends AppName,
  NS extends NamespaceOfApps,
  DBT extends DBType
> = Extract<AppEnvVars<AN, NS>, { dbType: DBT }>;

export type AppConfigs<
  AN extends AppName,
  DBT extends DBType,
  NS extends NamespaceOfApps
> = {
  kubeConfig: Settings<NoUnion<AN>>;
  envVars: Omit<EnvironmentVariables<AN, NS, DBT>, "dbType">;
  metadata: {
    name: AN;
    namespace: NS;
  };
};

export type NoUnion<T, U = T> = T extends U
  ? [U] extends [T]
    ? T
    : never
  : never;

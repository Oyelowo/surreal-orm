import { providerApplication } from '../shared/cluster';
import { ServiceDeployment } from '../shared/deployment';
import { graphqlPostgresSettings } from './settings';

export const graphqlPostgres = new ServiceDeployment(
  "graphql-postgres",
  graphqlPostgresSettings,
  { provider: providerApplication }
);

// export * from "./postgresHAdb";
export * from "./postgres";

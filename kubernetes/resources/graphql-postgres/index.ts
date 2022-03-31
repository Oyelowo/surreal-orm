import { applicationsDirectory } from '../shared/manifestsDirectory';
import { ServiceDeployment } from '../shared/deployment';
import { graphqlPostgresSettings } from './settings';

export const graphqlPostgres = new ServiceDeployment(
  "graphql-postgres",
  graphqlPostgresSettings,
  { provider: applicationsDirectory }
);

// export * from "./postgresHAdb";
export * from "./postgres";

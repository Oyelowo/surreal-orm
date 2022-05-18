import { ServiceDeployment } from '../../shared/deployment';
import { graphqlPostgresSettings } from './settings';

export const graphqlPostgres = new ServiceDeployment('graphql-postgres', graphqlPostgresSettings);

// export * from "./postgresHAdb";
export * from './postgres';

import { ServiceDeployment } from '../../shared/deployment.js';
import { graphqlPostgresSettings } from './settings.js';

export const graphqlPostgres = new ServiceDeployment('graphql-postgres', graphqlPostgresSettings);

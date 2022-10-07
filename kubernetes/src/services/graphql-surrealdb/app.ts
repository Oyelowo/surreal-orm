import { ServiceDeployment } from '../../shared/deployment.js';
import { graphqlSurrealdbSettings } from './settings.js';

export const graphqlSurrealdb = new ServiceDeployment('graphql-surrealdb', graphqlSurrealdbSettings);

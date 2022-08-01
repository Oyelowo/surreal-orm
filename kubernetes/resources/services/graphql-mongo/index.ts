import { ServiceDeployment } from '../../shared/deployment.js';
import { graphqlMongoSettings } from './settings.js';

export const graphqlMongo = new ServiceDeployment('graphql-mongo', graphqlMongoSettings);

export * from './mongodb.js';
export * from './redis.js';

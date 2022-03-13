import { provider } from '../shared/cluster';
import { ServiceDeployment } from '../shared/deployment';
import { graphqlMongoSettings } from './settings';

export const graphqlMongop = new ServiceDeployment(
  "graphql-mongo",
  graphqlMongoSettings,
  { provider }
);

export * from "./mongodb";
export * from "./redis";

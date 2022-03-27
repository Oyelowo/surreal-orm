import { providerApplication } from '../shared/cluster';
import { ServiceDeployment } from '../shared/deployment';
import { graphqlMongoSettings } from './settings';

export const graphqlMongo = new ServiceDeployment(
  "graphql-mongo",
  graphqlMongoSettings,
  { provider: providerApplication }
);

export * from "./mongodb";
export * from "./redis";

import { applicationsDirectory } from '../shared/manifestsDirectory';
import { ServiceDeployment } from '../shared/deployment';
import { graphqlMongoSettings } from './settings';

export const graphqlMongo = new ServiceDeployment(
  "graphql-mongo",
  graphqlMongoSettings,
  { provider: applicationsDirectory }
);

export * from "./mongodb";
export * from "./redis";

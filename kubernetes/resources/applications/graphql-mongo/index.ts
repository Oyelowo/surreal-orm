import { ServiceDeployment } from "../../shared/deployment";
import { graphqlMongoSettings } from "./settings";

export const graphqlMongo = new ServiceDeployment("graphql-mongo", graphqlMongoSettings);

export * from "./mongodb";
export * from "./redis";

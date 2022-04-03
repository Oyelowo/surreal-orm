import { graphqlMongoDirectoryPath, graphqlMongoProvider } from "./provider";
import { ServiceDeployment } from "../shared/deployment";
import { graphqlMongoSettings } from "./settings";
import { createArgocdApplication } from "../shared/createArgoApplicaiton";

export const graphqlMongo = new ServiceDeployment("graphql-mongo", graphqlMongoSettings, {
  provider: graphqlMongoProvider,
});

export const reactWebArgocdApplication = createArgocdApplication({
  metadata: {
    name: graphqlMongoSettings.metadata.name,
    namespace: graphqlMongoSettings.metadata.namespace,
  },
  provider: graphqlMongoProvider,
  pathToAppManifests: graphqlMongoDirectoryPath,
});

export * from "./mongodb";
export * from "./redis";

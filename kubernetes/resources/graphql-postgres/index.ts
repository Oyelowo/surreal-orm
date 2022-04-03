import { graphqlPostgresDirectoryPath, graphqlPostgresProvider } from "./provider";
import { ServiceDeployment } from "../shared/deployment";
import { graphqlPostgresSettings } from "./settings";
import { createArgocdApplication } from "../shared/createArgoApplicaiton";

export const graphqlPostgres = new ServiceDeployment("graphql-postgres", graphqlPostgresSettings, {
  provider: graphqlPostgresProvider,
});

export const reactWebArgocdApplication = createArgocdApplication({
  metadata: {
    name: graphqlPostgresSettings.metadata.name,
    namespace: graphqlPostgresSettings.metadata.namespace,
  },
  provider: graphqlPostgresProvider,
  pathToAppManifests: graphqlPostgresDirectoryPath,
});

// export * from "./postgresHAdb";
export * from "./postgres";

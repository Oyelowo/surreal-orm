import * as k8s from "@pulumi/kubernetes";
import { getPathToApplicationDir } from "../shared/manifestsDirectory";

export const graphqlPostgresDirectoryPath = getPathToApplicationDir("graphql-postgres");

export const graphqlPostgresProvider = new k8s.Provider("graphqlPostgresAppDir", {
  renderYamlToDirectory: graphqlPostgresDirectoryPath,
});

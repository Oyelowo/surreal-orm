import * as k8s from "@pulumi/kubernetes";
import { getPathToApplicationDir } from "../shared/manifestsDirectory";

export const graphqlMongoDirectoryPath = getPathToApplicationDir("graphql-mongo");

export const graphqlMongoProvider = new k8s.Provider(graphqlMongoDirectoryPath, {
  renderYamlToDirectory: graphqlMongoDirectoryPath,
});

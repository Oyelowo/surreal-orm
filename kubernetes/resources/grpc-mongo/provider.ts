import * as k8s from "@pulumi/kubernetes";
import { getPathToApplicationDir } from "../shared/manifestsDirectory";

export const grpcMongoDirectoryPath = getPathToApplicationDir("grpc-mongo");

export const grpcMongoProvider = new k8s.Provider(grpcMongoDirectoryPath, {
  renderYamlToDirectory: grpcMongoDirectoryPath,
});

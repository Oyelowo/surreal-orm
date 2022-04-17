import { ServiceDeployment } from "../../shared/deployment";
import { grpcMongoSettings } from "./settings";

export const grpcMongo = new ServiceDeployment("grpc-mongo", grpcMongoSettings);

export * from "./mongodb";

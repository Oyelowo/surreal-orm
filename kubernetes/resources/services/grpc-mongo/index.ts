import { ServiceDeployment } from '../../shared/deployment.js';
import { grpcMongoSettings } from './settings.js';

export const grpcMongo = new ServiceDeployment('grpc-mongo', grpcMongoSettings);

export * from './mongodb.js';

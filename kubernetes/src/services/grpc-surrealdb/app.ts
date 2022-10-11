import { ServiceDeployment } from '../../shared/deployment.js';
import { grpcSurrealdbSettings } from './settings.js';

export const grpcSurrealdb = new ServiceDeployment('grpc-surrealdb', grpcSurrealdbSettings);

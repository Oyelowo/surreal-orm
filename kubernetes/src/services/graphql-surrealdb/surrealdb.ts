import { ServiceDeployment } from '../../shared/deployment.js';

// Surrealdb is a compute/logic protocol layer using TiKV as persistent layer
// so, we can also deploy it as a kubernetes deployment or statefulset

import { AppConfigs } from '../../types/ownTypes.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';
import { PlainSecretsManager } from '../../../scripts/utils/plainSecretsManager.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';

const env = getEnvVarsForKubeManifests();

const secrets = new PlainSecretsManager('services', 'graphql-mongo', 'local').getSecrets();
type SurrealdbEnvVars = Record<string, string>;

export const surrealdbSettings: AppConfigs<'graphql-surrealdb', 'applications', SurrealdbEnvVars> = {
    kubeConfig: {
        requestMemory: '70Mi',
        requestCpu: '100m',
        limitMemory: '200Mi',
        limitCpu: '100m',
        replicaCount: 2,
        readinessProbePort: 8000,
        host: '0.0.0.0',
        image: `surrealdb/surrealdb:1.0.0-beta.8`,
        command: ['/surreal'],
        commandArgs: ['start', '--log', 'debug', '--user', 'root', '--pass', 'root', 'tikv://asts-pd:2379'],
    },
    envVars: {
        APP_ENVIRONMENT: env.ENVIRONMENT,
        APP_HOST: '0.0.0.0',
        APP_PORT: '8000',
        APP_EXTERNAL_BASE_URL: getIngressUrl({ environment: env.ENVIRONMENT }),
    },
    metadata: {
        name: 'graphql-surrealdb',
        namespace: 'applications',
    },
};

export const graphqlPostgres = new ServiceDeployment('graphql-surrealdb', surrealdbSettings);

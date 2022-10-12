import { ServiceDeployment } from '../../shared/deployment.js';
import { AppConfigs, AppEnvVars, TikVDbEnvVars } from '../../types/ownTypes.js';
import { getIngressUrl } from '../../infrastructure/ingress/hosts.js';
import { PlainSecretsManager } from '../../../scripts/utils/plainSecretsManager.js';
import { getEnvVarsForKubeManifests } from '../../shared/environmentVariablesForManifests.js';
import { graphqlSurrealdb } from './app.js';

const env = getEnvVarsForKubeManifests();

const secrets = new PlainSecretsManager('services', 'graphql-surrealdb', 'local').getSecrets();

type SurrealDbEnvVars = AppEnvVars & TikVDbEnvVars<'applications'>;

const surrealDbEnvVars: SurrealDbEnvVars = {
    APP_ENVIRONMENT: env.ENVIRONMENT,
    APP_HOST: '0.0.0.0',
    APP_PORT: '8000',
    APP_EXTERNAL_BASE_URL: getIngressUrl({ environment: env.ENVIRONMENT }),
    TIKV_HOST: 'tikv-pd.applications',
    TIKV_NAME: 'tikv',
    TIKV_PORT: '2379',
    TIKV_SERVICE_NAME: 'tikv',
    TIKV_STORAGE_CLASS: 'linode-block-storage-retain',
};

// Surrealdb is a compute/logic protocol layer using TiKV as persistent layer
// so, we can also deploy it as a kubernetes deployment or statefulset
export const surrealdbSettings: AppConfigs<'surrealdb', 'applications', SurrealDbEnvVars> = {
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
        // TODO: Change hardcoded credentials once surrealdb is 1.0
        commandArgs: [
            'start',
            '--log',
            'debug',
            '--user',
            secrets.SURREALDB_ROOT_USERNAME,
            '--pass',
            secrets.SURREALDB_ROOT_PASSWORD,
            `tikv://${surrealDbEnvVars.TIKV_HOST}:${surrealDbEnvVars.TIKV_PORT}`,
        ],
    },
    envVars: surrealDbEnvVars,
    metadata: {
        name: 'surrealdb',
        namespace: 'applications',
    },
};

export const surrealDbDeployment = new ServiceDeployment('surrealdb', surrealdbSettings);
surrealDbDeployment.setProvider(graphqlSurrealdb.getProvider());

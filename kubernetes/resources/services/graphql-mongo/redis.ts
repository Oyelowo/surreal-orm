import { IRedisbitnami } from '../../../generatedHelmChartsTsTypes/redisBitnami';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../../infrastructure/namespaces/util';
import { helmChartsInfo } from '../../shared/helmChartInfo';
import { DeepPartial } from '../../types/own-types';
import { getEnvironmentVariables } from '../../shared/validations';
import { graphqlMongo } from './index';
import { graphqlMongoSettings } from './settings';

const { envVars } = graphqlMongoSettings;

export const redisValues: DeepPartial<IRedisbitnami> = {
    architecture: 'standalone',
    fullnameOverride: envVars.REDIS_SERVICE_NAME,
    replica: {
        replicaCount: 1,
    },
    global: {
        redis: {
            password: envVars.REDIS_PASSWORD,
        },
        storageClass:
            getEnvironmentVariables().ENVIRONMENT === 'local' ? '' : graphqlMongoSettings.envVars.MONGODB_STORAGE_CLASS,
    },

    auth: {
        enabled: true,
        password: envVars.REDIS_PASSWORD,
    },
    master: {
        service: {
            type: 'ClusterIP',
            ports: {
                redis: Number(envVars.REDIS_PORT),
            },
        },
    },
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        redis: { chart, version },
    },
} = helmChartsInfo.bitnami;

export const graphqlMongoRedis = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: redisValues,
        namespace: namespaces.applications,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: graphqlMongo.getProvider() }
);

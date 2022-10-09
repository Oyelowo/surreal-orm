import { IMeilisearchmeilisearch } from '../../../generatedHelmChartsTsTypes/meilisearchMeilisearch.js';
import * as k8s from '@pulumi/kubernetes';
import { namespaces } from '../../infrastructure/namespaces/util.js';
import { helmChartsInfo } from '../../shared/helmChartInfo.js';
import { DeepPartial } from '../../types/ownTypes.js';
import { graphqlSurrealdb } from './app.js';
import { graphqlSurrealdbSettings } from './settings.js';

const { envVars } = graphqlSurrealdbSettings;

const mongoValues: DeepPartial<IMeilisearchmeilisearch> = {
    auth: {
        /* 
        For production deployment, the environment.MEILI_MASTER_KEY is required. 
        If MEILI_ENV is set to "production" without setting environment.MEILI_MASTER_KEY, 
        then this chart will automatically create a secure environment.MEILI_MASTER_KEY as a secret. 
        To get the value of this secret, you can read it with this command: kubectl get secret meilisearch-master-key 
        --template={{.data.MEILI_MASTER_KEY}} | base64 --decode. You can also use auth.existingMasterKeySecret 
        to use an existing secret that has the key MEILI_MASTER_KEY
        */
        // existingMasterKeySecret: ``,
    },
    replicaCount: 1,
    ingress: {
        className: '',
    },
    environment: {
        // MEILI_MASTER_KEY: '',
        MEILI_NO_ANALYTICS: false, //  Either production or development
        MEILI_ENV: 'production',
    },
};

// `http://${name}.${namespace}:${port}`;
const {
    repo,
    charts: {
        meilisearch: { chart, version },
    },
} = helmChartsInfo.meilisearch;
export const graphqlMongoMongodb = new k8s.helm.v3.Chart(
    chart,
    {
        chart,
        fetchOpts: {
            repo,
        },
        version,
        values: mongoValues,
        namespace: namespaces.applications,
        // By default Release resource will wait till all created resources
        // are available. Set this to true to skip waiting on resources being
        // available.
        skipAwait: false,
    },
    { provider: graphqlSurrealdb.getProvider() }
);

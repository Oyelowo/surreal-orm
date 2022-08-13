import { SealedSecretTemplate } from '../../../src/resources/types/sealedSecretTemplate.js';
import type { TKubeObject } from './kubeObject.js';
import p from 'node:path';
import sh from 'shelljs';
import { ResourceName } from '../../../src/resources/types/ownTypes.js';
import _ from 'lodash';
import z from 'zod';

const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = 'sealed-secrets';

type Props = {
    secretKubeObjects: TKubeObject<'Secret'>[];
    sealedSecretKubeObjects: TKubeObject<'SealedSecret'>[];
};

/*
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
export function mergeUnsealedSecretToSealedSecret(props: Props): void {
    for (const secret of props.secretKubeObjects) {
        mergeUnsealedSecretToSealedSecretHelper({
            secretKubeObject: secret,
            sealedSecretKubeObjects: props.sealedSecretKubeObjects,
        });
    }
}

function mergeUnsealedSecretToSealedSecretHelper({
    sealedSecretKubeObjects,
    secretKubeObject,
}: {
    secretKubeObject: TKubeObject<'Secret'>;
    sealedSecretKubeObjects: TKubeObject<'SealedSecret'>[];
}): void {
    const { data, selectedSecretsForUpdate, metadata, path } = secretKubeObject;
    const { name, namespace /* annotations */ } = metadata;

    if (!name && namespace) {
        throw new Error('Name and namespace not provided in the secret');
    }

    // Get corresponding previously generated sealed secrets info(if it exists).
    const matchesUnsealedSecret = ({ metadata: m }: TKubeObject): boolean =>
        m.name === name && m.namespace === namespace;
    const existingSealedSecretJsonData = sealedSecretKubeObjects?.find(matchesUnsealedSecret);

    const sealSecretValue = (secretValue: string): string => {
        return sh
            .exec(
                `echo ${secretValue} | base64 -d | kubeseal --controller-name=${SEALED_SECRETS_CONTROLLER_NAME} \
            --raw --from-file=/dev/stdin --namespace ${namespace} \
            --name ${name}`
            )
            .stdout.trim();
    };

    const secretData = data ?? {};

    // Pick only selected secrets for encytption
    const filteredSecretData = _.pickBy(secretData, (_v, k) => selectedSecretsForUpdate?.includes(k));
    const updatedSealedSecretsData = _.mapValues(filteredSecretData, sealSecretValue);

    // Merge new secrets with old
    const encryptedDataa: Record<string, unknown> = {
        ...existingSealedSecretJsonData?.spec?.encryptedData,
        ...updatedSealedSecretsData,
    };

    const recordSchema = z.record(z.string());
    const encryptedData = recordSchema.parse(encryptedDataa);

    // Remove stale/unsed encrypted secret
    const unfilteredSecretKeys = Object.keys(secretData) ?? [];
    const mergedEncryptedData = _.pickBy(encryptedData, (_v, k) => unfilteredSecretKeys.includes(k));

    // Update sealed secret object to be converted to yaml
    const updatedSealedSecrets: SealedSecretTemplate = {
        kind: 'SealedSecret',
        apiVersion: 'bitnami.com/v1alpha1',
        metadata: {
            name,
            namespace,
            annotations: {
                'sealedsecrets.bitnami.com/managed': 'true',
                ...existingSealedSecretJsonData?.metadata.annotations,
            },
            ...existingSealedSecretJsonData?.metadata,
        },
        spec: {
            encryptedData: mergedEncryptedData,
            template: {
                ...existingSealedSecretJsonData?.spec?.template,
                metadata,
                type: secretKubeObject.type,
            },
        },
    };

    // GET SEALED SECRET PATH USING UNSEALED SECRET PATH
    const appManifestsDir = p.dirname(path);
    // The path format is: kubernetes/generatedManifests/production/applications/graphql-mongo/1-manifest
    // and we want as basedir: kubernetes/generatedManifests/production/applications/graphql-mongo
    const appBaseDir = p.join(appManifestsDir, '..');
    const sealedSecretDir = p.join(appBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
    sh.mkdir(sealedSecretDir);
    const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-secret-${name}-${namespace}.yaml`);

    // sh.exec(`echo '${yaml.dump(updatedSealedSecrets)}' > ${sealedSecretFilePath}`);
    sh.exec(`echo '${JSON.stringify(updatedSealedSecrets)}' | yq -P '.' -o yaml > ${sealedSecretFilePath}`);
}

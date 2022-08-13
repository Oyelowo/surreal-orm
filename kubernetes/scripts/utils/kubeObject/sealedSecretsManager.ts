import { SealedSecretTemplate } from '../../../src/resources/types/sealedSecretTemplate.js';
import type { TKubeObject } from './kubeObject.js';
import p from 'node:path';
import sh from 'shelljs';
import { ResourceName } from '../../../src/resources/types/ownTypes.js';
import _ from 'lodash';
import z from 'zod';
import yaml from 'yaml';

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
        const { name, namespace } = secret.metadata;
        const updatedSealedSecret = mergeUnsealedSecretToSealedSecretHelper({
            secretKubeObject: secret,
            existingSealedSecretKubeObjects: props.sealedSecretKubeObjects,
        });

        const sealedSecretDir = p.join(secret.resourceBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
        sh.mkdir('-p', sealedSecretDir);

        const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-secret-${name}-${namespace}.yaml`);

        sh.exec(`echo '${yaml.stringify(updatedSealedSecret)}' > ${sealedSecretFilePath}`);
    }
}

function sealSecretValue({ namespace, name, secretValue }: { namespace: string; name: string; secretValue: string }) {
    return sh
        .exec(
            `echo ${secretValue} | base64 - d | kubeseal--controller - name=${SEALED_SECRETS_CONTROLLER_NAME} \
            --raw --from - file=/dev/stdin --namespace ${namespace} \
            --name ${name}`
        )
        .stdout.trim();
}

function mergeUnsealedSecretToSealedSecretHelper({
    existingSealedSecretKubeObjects,
    secretKubeObject,
}: {
    secretKubeObject: TKubeObject<'Secret'>;
    existingSealedSecretKubeObjects: TKubeObject<'SealedSecret'>[];
}): SealedSecretTemplate {
    const { data, selectedSecretsForUpdate, metadata, path } = secretKubeObject;
    const { name, namespace /* annotations */ } = metadata;

    if (!name && namespace) {
        throw new Error('Name and namespace not provided in the secret');
    }

    // Get corresponding previously generated sealed secrets info(if it exists).
    const matchesUnsealedSecret = ({ metadata: m }: TKubeObject): boolean =>
        m.name === name && m.namespace === namespace;
    const existingSealedSecretJsonData = existingSealedSecretKubeObjects?.find(matchesUnsealedSecret);

    const secretData = data ?? {};

    // Pick only selected secrets for encytption
    const filteredSecretData = _.pickBy(secretData, (_v, k) => selectedSecretsForUpdate?.includes(k));
    const updatedSealedSecretsData = _.mapValues(
        filteredSecretData,
        (secretValue) => `sealSecretValue({ namespace, name, secretValue })`
    );

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

    return updatedSealedSecrets;
}

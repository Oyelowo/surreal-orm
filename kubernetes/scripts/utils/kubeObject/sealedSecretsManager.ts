import { SealedSecretTemplate } from '../../../src/resources/types/sealedSecretTemplate.js';
import type { TKubeObject } from './kubeObject.js';
import p from 'node:path';
import sh from 'shelljs';
import type { ResourceName } from '../../../src/resources/types/ownTypes.js';
import _ from 'lodash';
import z from 'zod';
import yaml from 'yaml';
import { Namespace } from '../../../src/resources/infrastructure/namespaces/util.js';

const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = 'sealed-secrets';

type OnSealSecretValue = ({
    namespace,
    name,
    secretValue,
}: {
    namespace: Namespace;
    name: string;
    secretValue: string;
}) => string;

type Props = {
    secretKubeObjects: TKubeObject<'Secret'>[];
    existingSealedSecretKubeObjects: TKubeObject<'SealedSecret'>[];
    onSealSecretValue: OnSealSecretValue;
};

/*
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
export function mergeUnsealedSecretToSealedSecret(props: Props): void {
    for (const secret of props.secretKubeObjects) {
        const { name, namespace } = secret.metadata;
        const updatedSealedSecret = updateExistingSealedSecret({
            secretKubeObject: secret,
            existingSealedSecretKubeObjects: props.existingSealedSecretKubeObjects,
            onSealSecretValue: props.onSealSecretValue,
        });

        const sealedSecretDir = p.join(secret.resourceBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
        sh.mkdir('-p', sealedSecretDir);

        const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-secret-${name}-${namespace}.yaml`);
        sh.exec(`echo '${yaml.stringify(updatedSealedSecret)}' > ${sealedSecretFilePath}`);
    }
}

function updateExistingSealedSecret({
    existingSealedSecretKubeObjects,
    secretKubeObject,
    onSealSecretValue,
}: {
    secretKubeObject: TKubeObject<'Secret'>;
    existingSealedSecretKubeObjects: TKubeObject<'SealedSecret'>[];
    onSealSecretValue: OnSealSecretValue;
}): SealedSecretTemplate {
    const { data: secretData, selectedSecretsForUpdate, metadata } = secretKubeObject;
    const { name, namespace /* annotations */ } = metadata;

    if (!name || !namespace) {
        throw new Error('Name and namespace not provided in the secret');
    }

    // Get corresponding previously generated sealed secrets info(if it exists).
    const matchesUnsealedSecret = ({ metadata: m }: TKubeObject): boolean =>
        m.name === name && m.namespace === namespace;
    const existingSealedSecretJsonData = existingSealedSecretKubeObjects?.find(matchesUnsealedSecret);

    // Pick only selected secrets for encytption
    const filteredSecretData = _.pickBy(secretData, (_v, k): boolean => !!selectedSecretsForUpdate?.includes(k));
    const updatedSealedSecretsData = _.mapValues(filteredSecretData, (secretValue): string =>
        onSealSecretValue({ namespace, name, secretValue: String(secretValue) })
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

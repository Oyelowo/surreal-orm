import { TSecretKubeObject } from './kubeObject';
// import { ManifestsManager, SealedSecretKubeObject } from './kubeObject';
import { SealedSecretTemplate } from '../../../resources/types/sealedSecretTemplate';
import { TKubeObject, TSealedSecretKubeObject } from './kubeObject';
import p from 'path';
import sh from 'shelljs';
import { ResourceName } from '../../../resources/types/own-types';
import _ from 'lodash';

const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = 'sealed-secrets';

type Props = {
    secretKubeObjects: TSecretKubeObject[];
    sealedSecretKubeObjects: TSealedSecretKubeObject[];
};

/*
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
export async function mergeUnsealedSecretToSealedSecret(props: Props) {
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
    secretKubeObject: TSecretKubeObject;
    sealedSecretKubeObjects: TSealedSecretKubeObject[];
}): void {
    const { data, stringData, selectedSecretsForUpdate } = secretKubeObject;
    const { name, namespace /* annotations */ } = secretKubeObject.metadata;

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
                `echo -n ${secretValue} | kubeseal --controller-name=${SEALED_SECRETS_CONTROLLER_NAME} \
            --raw --from-file=/dev/stdin --namespace ${namespace} \
            --name ${name}`
            )
            .stdout.trim();
    };

    const secretData = stringData ?? data ?? {};

    // Pick only selected secrets for encytption
    const filteredSecretData = _.pickBy(secretData, (_v, k) => selectedSecretsForUpdate?.includes(k));
    const updatedSealedSecretsData = _.mapValues(filteredSecretData, sealSecretValue);

    // Merge new secrets with old
    const encryptedData: Record<string, string> = {
        // ...existingSealedSecretJsonData?.spec?.encryptedData,
        // ...updatedSealedSecretsData,
    };

    // Remove stale/unsed encrypted secret
    const unfilteredSecretKeys = Object.keys(secretData) ?? [];
    const mergedEncryptedData = _.pickBy(encryptedData, (_v, k) => unfilteredSecretKeys.includes(k));

    // Update sealed secret object to be converted to yaml
    const updatedSealedSecrets: SealedSecretTemplate = {
        kind: 'SealedSecret',
        apiVersion: 'bitnami.com/v1alpha1',
        metadata: {
            name: secretKubeObject.metadata.name,
            namespace: secretKubeObject.metadata.namespace,
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
                data: null,
                metadata: secretKubeObject.metadata,
                type: secretKubeObject.type,
            },
        },
    };

    // GET SEALED SECRET PATH USING UNSEALED SECRET PATH
    const appManifestsDir = p.dirname(secretKubeObject.path);
    // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
    // and we want as basedir: kubernetes/manifests/generated/production/applications/graphql-mongo
    const appBaseDir = p.join(appManifestsDir, '..');
    const sealedSecretDir = p.join(appBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
    sh.mkdir(sealedSecretDir);
    const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-secret-${name}-${namespace}.yaml`);

    // sh.exec(`echo '${yaml.dump(updatedSealedSecrets)}' > ${sealedSecretFilePath}`);
    sh.exec(`echo '${JSON.stringify(updatedSealedSecrets)}' | yq -P '.' -o yaml > ${sealedSecretFilePath}`);
}

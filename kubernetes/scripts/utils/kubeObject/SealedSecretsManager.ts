import { TSecretKubeObject } from './kubeObject';
// import { ManifestsManager, SealedSecretKubeObject } from './kubeObject';
import { SealedSecretTemplate } from '../../../resources/types/sealedSecretTemplate';
import { TKubeObject, TSealedSecretKubeObject } from './kubeObject';
import p from 'path';
import yaml from 'js-yaml';
import sh from 'shelljs';
import { Environment, ResourceName } from '../../../resources/types/own-types';
import _ from 'lodash';

const SEALED_SECRETS_CONTROLLER_NAME: ResourceName = 'sealed-secrets';


type Props = {
    secretKubeObjects: TSecretKubeObject[];
    sealedSecretKubeObjects: TSealedSecretKubeObject[];
};


export class SealedSecretsMerger {
    /*
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
    static mergeUnsealedSecretToSealedSecret = async (props: Props) => {
        for (let unsealedSecret of props.secretKubeObjects) {
            SealedSecretsMerger.#mergeUnsealedSecretToSealedSecretHelper({
                unsealedSecretOne: unsealedSecret,
                sealedSecretAll: props.sealedSecretKubeObjects,
            });
        }
    }


    static #mergeUnsealedSecretToSealedSecretHelper = ({ sealedSecretAll, unsealedSecretOne }: {
        unsealedSecretOne: TKubeObject;
        sealedSecretAll: TSealedSecretKubeObject[];
    }): void => {
        const { data, stringData } = unsealedSecretOne;
        const { name, namespace, /* annotations */ } = unsealedSecretOne.metadata;

        if (!name && namespace) {
            throw new Error('Name and namespace not provided in the secret');
        }

        // Get corresponding previously(if it exists) generated sealed secrets info.
        const matchesUnsealedSecret = ({ metadata: m }: TKubeObject): boolean =>
            m.name === name && m.namespace === namespace;
        const existingSealedSecretJsonData = sealedSecretAll?.find(matchesUnsealedSecret);

        const sealSecretValue = (secretValue: string): string => {
            return sh
                .exec(
                    `echo -n ${secretValue} | kubeseal --controller-name=${SEALED_SECRETS_CONTROLLER_NAME} \
            --raw --from-file=/dev/stdin --namespace ${namespace} \
            --name ${name}`
                )
                .stdout.trim();
        };

        const dataToSeal = stringData ?? data ?? {};
        const filteredSealedSecretsData = _.mapValues(dataToSeal, sealSecretValue) as unknown as Record<
            string,
            string | null
        >;

        // Update sealed secret object to be converted to yaml
        const updatedSealedSecrets: SealedSecretTemplate = {
            // For some reason, typescript is not detecting the correct type here.
            kind: 'SealedSecret',
            apiVersion: 'bitnami.com/v1alpha1',
            metadata: {
                name: unsealedSecretOne.metadata.name,
                namespace: unsealedSecretOne.metadata.namespace,
                annotations: {
                    'sealedsecrets.bitnami.com/managed': 'true',
                    ...existingSealedSecretJsonData?.metadata.annotations,
                },
                ...existingSealedSecretJsonData?.metadata,
            },
            spec: {
                encryptedData: {
                    ...existingSealedSecretJsonData?.spec?.encryptedData,
                    ...filteredSealedSecretsData,
                },
                template: {
                    ...existingSealedSecretJsonData?.spec?.template,
                    data: null,
                    metadata: unsealedSecretOne.metadata,
                    type: unsealedSecretOne.type,
                },
            },
        };

        // GET SEALED SECRET PATH USING UNSEALED SECRET PATH
        const appManifestsDir = p.dirname(unsealedSecretOne.path);
        // The path format is: kubernetes/manifests/generated/production/applications/graphql-mongo/1-manifest
        // and we want as basedir: kubernetes/manifests/generated/production/applications/graphql-mongo
        const appBaseDir = p.join(appManifestsDir, '..');
        const sealedSecretDir = p.join(appBaseDir, SEALED_SECRETS_CONTROLLER_NAME);
        sh.mkdir(sealedSecretDir);
        const sealedSecretFilePath = p.join(sealedSecretDir, `sealed-secret-${name}-${namespace}.yaml`);

        // sh.exec(`echo '${yaml.dump(updatedSealedSecrets)}' > ${sealedSecretFilePath}`);
        sh.exec(`echo '${JSON.stringify(updatedSealedSecrets)}' | yq -P '.' -o yaml > ${sealedSecretFilePath}`);
    }

}







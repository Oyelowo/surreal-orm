import c from 'chalk';
import sh from 'shelljs';
import { getGeneratedEnvManifestsDir } from '../../resources/shared/manifestsDirectory';
import { Environment } from '../../resources/types/own-types';
import { getSecretPathsInfo, SEALED_SECRETS_CONTROLLER_NAME } from './sealedSecrets';
import { getKubernetesSecretsPaths } from './shared';

/*
GENERATE BITNAMI'S SEALED SECRET FROM PLAIN SECRETS MANIFESTS GENERATED USING PULUMI.
These secrets are encrypted using the bitnami sealed secret controller running in the cluster
you are at present context
*/
export interface GenSealedSecretsProps {
    environment: Environment;
}

// CONSIDER: Prompt user to specify which apps secrets they're looking to update
export async function generateAllSealedSecrets({ environment }: GenSealedSecretsProps) {
    const unsealedSecretsFilePathsForEnv = getKubernetesSecretsPaths({
        environmentManifestsDir: getGeneratedEnvManifestsDir(environment),
    });

    const generateSealedSecret = (kubernetesSecretPath: string) => {
        const { sealedSecretDir, sealedSecretFilePath } = getSecretPathsInfo({
            unsealedSecretFilePath: kubernetesSecretPath,
        });

        sh.mkdir(sealedSecretDir);

        sh.echo(c.blueBright(`Generating sealed secret ${kubernetesSecretPath} \n to \n ${sealedSecretFilePath}`));

        const kubeSeal = sh.exec(
            `kubeseal --controller-name ${SEALED_SECRETS_CONTROLLER_NAME} < ${kubernetesSecretPath} -o yaml >${sealedSecretFilePath}`,
            { silent: true }
        );

        //  CONSIDER?: inject annotations that this is being managed by the sealed secrets controller.
        sh.echo(c.greenBright(kubeSeal.stdout));
        if (kubeSeal.stderr) {
            sh.echo(`Error sealing secrets: ${c.redBright(kubeSeal.stderr)}`);
            sh.exit(1);
            return;
        }

        sh.echo(c.greenBright('Successfully generated sealed secret at', kubernetesSecretPath));
    }

    unsealedSecretsFilePathsForEnv?.forEach(generateSealedSecret);
}

